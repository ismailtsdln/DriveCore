use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ControlCommand {
    /// Steering angle in radians (-max to +max)
    pub steering: f32,
    /// Throttle percentage (0.0 to 1.0)
    pub throttle: f32,
    /// Brake percentage (0.0 to 1.0)
    pub brake: f32,
}

impl Default for ControlCommand {
    fn default() -> Self {
        Self {
            steering: 0.0,
            throttle: 0.0,
            brake: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VehicleType {
    KiaSoul,
    TeslaModel3,
    ToyotaCorolla,
    VWGolf,
    Simulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub speed_kmh: f32,
    pub steering_angle: f32,
    pub battery_level: Option<f32>,
    pub timestamp: u64,
}
