use async_trait::async_trait;
use drivecore_common::{ControlCommand, Result, VehicleState};

/// The core trait that all vehicle firmware implementations must support.
#[async_trait]
pub trait VehicleInterface: Send + Sync {
    /// Send control commands to the vehicle
    async fn send_control(&mut self, command: ControlCommand) -> Result<()>;

    /// Read the current state of the vehicle
    async fn read_state(&self) -> Result<VehicleState>;

    /// Initialize the connection to the vehicle
    async fn connect(&mut self) -> Result<()>;

    /// Closest the connection safely
    async fn disconnect(&mut self) -> Result<()>;
}
