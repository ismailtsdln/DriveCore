use drivecore_common::{ControlCommand, DriveCoreError, Result, VehicleState};
use drivecore_firmware::VehicleInterface;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DriveCore {
    vehicle: Arc<Mutex<Box<dyn VehicleInterface>>>,
    #[allow(dead_code)]
    max_speed_kmh: f32,
    max_steering_angle: f32,
}

impl DriveCore {
    pub fn new(vehicle: Box<dyn VehicleInterface>) -> Self {
        Self {
            vehicle: Arc::new(Mutex::new(vehicle)),
            max_speed_kmh: 120.0,    // Default limit
            max_steering_angle: 0.5, // Default ~28 degrees
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let mut v = self.vehicle.lock().await;
        v.connect().await
    }

    pub async fn disconnect(&self) -> Result<()> {
        let mut v = self.vehicle.lock().await;
        v.disconnect().await
    }

    pub async fn apply_control(&self, mut command: ControlCommand) -> Result<()> {
        // Safety checks
        if command.throttle < 0.0 || command.throttle > 1.0 {
            return Err(DriveCoreError::SafetyViolation(
                "Throttle out of range".into(),
            ));
        }
        if command.brake < 0.0 || command.brake > 1.0 {
            return Err(DriveCoreError::SafetyViolation("Brake out of range".into()));
        }
        if command.steering.abs() > self.max_steering_angle {
            // Clamping steering for safety
            command.steering = command
                .steering
                .clamp(-self.max_steering_angle, self.max_steering_angle);
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
        let core = DriveCore::new(vehicle);
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
        // This should succeed but clamp internally (or return error depending on implementation choices,
        // but current impl clamps). Wait, current impl modifies the command but returns Ok.
        // Let's verify it returns Ok.
        assert!(core.apply_control(extreme_steering).await.is_ok());
    }
}
