use async_trait::async_trait;
use drivecore_common::{ControlCommand, Result, VehicleState};
use drivecore_firmware::VehicleInterface;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct KiaSoul {
    connected: bool,
    current_command: ControlCommand,
}

impl KiaSoul {
    pub fn new() -> Self {
        Self {
            connected: false,
            current_command: ControlCommand::default(),
        }
    }
}

#[async_trait]
impl VehicleInterface for KiaSoul {
    async fn connect(&mut self) -> Result<()> {
        self.connected = true;
        // In real life, init CAN bus connection here
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.connected = false;
        Ok(())
    }

    async fn send_control(&mut self, command: ControlCommand) -> Result<()> {
        if !self.connected {
            return Err(drivecore_common::DriveCoreError::CommunicationError(
                "Not connected".to_string(),
            ));
        }
        // Map command to Kia Soul CAN frames
        self.current_command = command;
        Ok(())
    }

    async fn read_state(&self) -> Result<VehicleState> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(VehicleState {
            speed_kmh: self.current_command.throttle * 150.0, // Kia Soul is faster :)
            steering_angle: self.current_command.steering,
            battery_level: Some(92.0),
            timestamp,
        })
    }
}
