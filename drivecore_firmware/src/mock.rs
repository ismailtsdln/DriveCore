use crate::interface::VehicleInterface;
use async_trait::async_trait;
use drivecore_common::{ControlCommand, Result, VehicleState};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MockVehicle {
    connected: bool,
    current_command: ControlCommand,
}

impl MockVehicle {
    pub fn new() -> Self {
        Self {
            connected: false,
            current_command: ControlCommand::default(),
        }
    }
}

#[async_trait]
impl VehicleInterface for MockVehicle {
    async fn connect(&mut self) -> Result<()> {
        self.connected = true;
        println!("MockVehicle connected.");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.connected = false;
        println!("MockVehicle disconnected.");
        Ok(())
    }

    async fn send_control(&mut self, command: ControlCommand) -> Result<()> {
        if !self.connected {
            return Err(drivecore_common::DriveCoreError::CommunicationError(
                "Not connected".to_string(),
            ));
        }
        self.current_command = command;
        // println!("MockVehicle received command: {:?}", command);
        Ok(())
    }

    async fn read_state(&self) -> Result<VehicleState> {
        if !self.connected {
            return Err(drivecore_common::DriveCoreError::CommunicationError(
                "Not connected".to_string(),
            ));
        }

        // Return a simulated state based on the last command
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(VehicleState {
            speed_kmh: self.current_command.throttle * 100.0, // Minimal simulation
            steering_angle: self.current_command.steering,
            battery_level: Some(85.0),
            timestamp,
        })
    }
}
