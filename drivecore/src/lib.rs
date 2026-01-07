//! # DriveCore
//!
//! `drivecore` is the central controller library for the DriveCore autonomous vehicle SDK.
//! It handles safety checks, state management, and interaction with vehicle firmware.
//!
//! ## Safety Features
//!
//! - **Rate Limiting**: Prevents command flooding.
//! - **Range Validation**: Ensures throttle/brake are within [0.0, 1.0].
//! - **Watchdog**: Engines an emergency stop if no heartbeat is received within `timeout_ms`.
//!
//! ## Example
//!
//! ```rust,no_run
//! use drivecore::DriveCore;
//! use drivecore_common::DriveCoreConfig;
//! // use drivecore_firmware::MockVehicle;
//!
//! #[tokio::main]
//! async fn main() {
//!     // let vehicle = Box::new(MockVehicle::new());
//!     // let core = DriveCore::new(vehicle, DriveCoreConfig::default());
//!     // core.connect().await.unwrap();
//! }
//! ```

pub mod controller;

pub use controller::DriveCore;
