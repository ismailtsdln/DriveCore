use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveCoreConfig {
    pub vehicle: VehicleConfig,
    pub safety: SafetyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    pub max_speed_kmh: f32,
    pub max_steering_angle: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    /// Max time (ms) without a command before emergency stop
    pub watchdog_timeout_ms: u64,
}

impl Default for DriveCoreConfig {
    fn default() -> Self {
        Self {
            vehicle: VehicleConfig {
                max_speed_kmh: 120.0,
                max_steering_angle: 0.5,
            },
            safety: SafetyConfig {
                watchdog_timeout_ms: 500,
            },
        }
    }
}
