use drivecore_common::{ControlCommand, DriveCoreConfig, DriveCoreError, Result, VehicleState};
use drivecore_firmware::VehicleInterface;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

// Need to allow clean reuse of the core controller
#[derive(Clone)]
pub struct DriveCore {
    vehicle: Arc<Mutex<Box<dyn VehicleInterface>>>,
    config: DriveCoreConfig,
    last_heartbeat: Arc<Mutex<SystemTime>>,
}

impl DriveCore {
    pub fn new(vehicle: Box<dyn VehicleInterface>, config: DriveCoreConfig) -> Self {
        let core = Self {
            vehicle: Arc::new(Mutex::new(vehicle)),
            config,
            last_heartbeat: Arc::new(Mutex::new(SystemTime::now())),
        };
        core.start_watchdog();
        core
    }

    fn start_watchdog(&self) {
        let vehicle = self.vehicle.clone();
        let last_heartbeat = self.last_heartbeat.clone();
        let timeout_ms = self.config.safety.watchdog_timeout_ms;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
            loop {
                interval.tick().await;
                let elapsed = {
                    let last = last_heartbeat.lock().await;
                    last.elapsed().unwrap_or_default().as_millis() as u64
                };

                if elapsed > timeout_ms {
                    warn!(
                        "Watchdog triggered! Last heartbeat {}ms ago. Engaging emergency stop.",
                        elapsed
                    );
                    let mut v = vehicle.lock().await;
                    let stop_cmd = ControlCommand {
                        throttle: 0.0,
                        brake: 1.0,
                        steering: 0.0,
                    };
                    if let Err(e) = v.send_control(stop_cmd).await {
                        error!("Failed to send emergency stop: {}", e);
                    }
                }
            }
        });
    }

    pub async fn heartbeat(&self) {
        let mut last = self.last_heartbeat.lock().await;
        *last = SystemTime::now();
        // debug!("Heartbeat received");
    }

    pub async fn connect(&self) -> Result<()> {
        info!("Connecting to vehicle backend...");
        let mut v = self.vehicle.lock().await;
        v.connect().await
    }

    pub async fn disconnect(&self) -> Result<()> {
        info!("Disconnecting from vehicle backend...");
        let mut v = self.vehicle.lock().await;
        v.disconnect().await
    }

    pub async fn apply_control(&self, mut command: ControlCommand) -> Result<()> {
        self.heartbeat().await; // Update heartbeat on control

        // Safety checks
        if command.throttle < 0.0 || command.throttle > 1.0 {
            error!("Safety violation: Throttle {}", command.throttle);
            return Err(DriveCoreError::SafetyViolation(
                "Throttle out of range".into(),
            ));
        }
        if command.brake < 0.0 || command.brake > 1.0 {
            error!("Safety violation: Brake {}", command.brake);
            return Err(DriveCoreError::SafetyViolation("Brake out of range".into()));
        }

        let max_steer = self.config.vehicle.max_steering_angle;
        if command.steering.abs() > max_steer {
            warn!(
                "Clamping steering from {} to {}",
                command.steering, max_steer
            );
            command.steering = command.steering.clamp(-max_steer, max_steer);
        }

        let mut v = self.vehicle.lock().await;
        v.send_control(command).await
    }

    pub async fn get_state(&self) -> Result<VehicleState> {
        let v = self.vehicle.lock().await;
        v.read_state().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use drivecore_firmware::MockVehicle;

    #[tokio::test]
    async fn test_safety_limits() {
        let vehicle = Box::new(MockVehicle::new());
        let core = DriveCore::new(vehicle, DriveCoreConfig::default());
        core.connect().await.unwrap();

        // Test normal command
        let cmd = ControlCommand {
            steering: 0.1,
            throttle: 0.5,
            brake: 0.0,
        };
        assert!(core.apply_control(cmd).await.is_ok());

        // Test throttle violation
        let bad_throttle = ControlCommand {
            steering: 0.0,
            throttle: 1.5,
            brake: 0.0,
        };
        assert!(core.apply_control(bad_throttle).await.is_err());

        // Test steering clamping
        let extreme_steering = ControlCommand {
            steering: 1.0, // Limit is 0.5
            throttle: 0.0,
            brake: 0.0,
        };
        // This should succeed but clamp internally
        assert!(core.apply_control(extreme_steering).await.is_ok());
    }
}
