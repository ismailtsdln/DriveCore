# DriveCore

<div align="center">

![DriveCore Logo](https://via.placeholder.com/150/000000/FFFFFF?text=DriveCore)

**A modern, safe, and modular vehicle control SDK for autonomous system development.**  
*Built with Rust for performance and reliability.*

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/ismailtsdln/DriveCore/actions/workflows/ci.yml/badge.svg)](https://github.com/ismailtsdln/DriveCore/actions)

</div>

---

## 🚀 Overview

**DriveCore** is a next-generation SDK designed to replace legacy C/C++ vehicle control systems. It provides a **Unified Control API** that abstracts hardware differences, allowing developers to write safe, vehicle-agnostic autonomous driving software.

Whether you are controlling a **Tesla Model 3**, a **Kia Soul**, or a custom robotics platform, DriveCore provides the safety assurances and tooling you need.

## ✨ Key Features

### 🛡️ Safety First
- **Watchdog Timer**: Auto-engages emergency brakes if the controller heartbeat is lost (>500ms).
- **Range Validation**: Strict clamping and validation of steering, throttle, and brake inputs.
- **Type Safety**: leveraged Rust's type system to prevent common memory safety errors.

### 🔌 Modular Architecture
- **`drivecore`**: The brain. Handles state, safety logic, and high-level control.
- **`drivecore_firmware`**: Hardware Abstraction Layer (HAL).
- **`drivecore_vehicle`**: Plug-and-play modules for specific vehicles (e.g., *Kia Soul*, *Toyota Corolla*).
- **`drivecore_cli`**: Advanced CLI for monitoring, control, and CAN analysis.

### ⚙️ Production Ready
- **Structured Logging**: Professional telemetry via `tracing` (JSON/stdout).
- **Configuration**: TOML-based configuration for safety limits and vehicle parameters.
- **CAN Analysis**: Built-in tools to sniff and decode CAN bus traffic.

## 📦 Architecture

```mermaid
graph TD
    User[User Code / CLI] -->|ControlCommand| Core[DriveCore Controller]
    Core -->|Checks & Safety| Watchdog[Safety Watchdog]
    Core -->|Normalized Cmd| HAL[Firmware Abstraction]
    HAL -->|CAN Frame| Vehicle[Specific Vehicle (e.g., Kia Soul)]
    Vehicle -->|CAN| Hardware[Hardware/Simulator]
```

## 🛠️ Getting Started

### Prerequisites
- **Rust Toolchain**: `stable` (Install via [rustup.rs](https://rustup.rs))

### Installation

Clone the repository:
```bash
git clone https://github.com/ismailtsdln/DriveCore.git
cd DriveCore
```

Build the workspace:
```bash
cargo build --release
```

## 🎮 Usage via CLI

DriveCore comes with a powerful CLI tool `drivecore_cli` for testing and development.

### 1. Monitor Vehicle State
View real-time telemetry (Speed, Steering, Battery) in a formatted dashboard.
```bash
cargo run --bin drivecore_cli -- monitor
```

### 2. Send Control Commands
Send manual commands to the connected vehicle.
```bash
# Set throttle to 50%, Steering to 0.1 rad
cargo run --bin drivecore_cli -- control --throttle 0.5 --steering 0.1
```

### 3. Analyze CAN Traffic
Sniff raw CAN frames (useful for reverse engineering).
```bash
cargo run --bin drivecore_cli -- analyze
```

## ⚙️ Configuration

Create a `config.toml` (optional, defaults provided):

```toml
[vehicle]
max_speed_kmh = 150.0
max_steering_angle = 0.52 # ~30 degrees

[safety]
watchdog_timeout_ms = 250 # Strict timeout
```

## 🧪 Testing & Verification

Run the comprehensive test suite:
```bash
cargo test
```

DriveCore uses **Mock Vehicles** for unit testing to ensure safety logic holds true even without hardware attached.

## 🤝 Contributing

Contributions are welcome!
1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---
<div align="center">
    <sub>Built with ❤️ by the DriveCore Team</sub>
</div>
