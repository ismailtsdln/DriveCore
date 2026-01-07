use thiserror::Error;

#[derive(Error, Debug)]
pub enum DriveCoreError {
    #[error("Communication error: {0}")]
    CommunicationError(String),
    #[error("hardware error: {0}")]
    HardwareError(String),
    #[error("Safety violation: {0}")]
    SafetyViolation(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Vehicle not supported: {0}")]
    UnsupportedVehicle(String),
}

pub type Result<T> = std::result::Result<T, DriveCoreError>;
