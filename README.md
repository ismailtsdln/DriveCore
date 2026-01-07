# DriveCore

A modern, safe, and modular vehicle control SDK for autonomous system development — built with Rust.

## 🚀 Features

- **Unified control API** for steering, throttle, brake.
- **Multi-vehicle support** (Kia Soul, Tesla Model 3, etc. - extensible).
- **Live CAN monitoring** (simulated initially).
- **Safety and command filtering systems** (rate limiting, range validation).
- **Built-in test suite**.

## 📦 Architecture

- 🔹 **drivecore**: Core API and logic.
- 🔹 **drivecore_firmware**: Firmware abstraction layer and hardware interfaces.
- 🔹 **drivecore_local**: Shared types and traits.
- 🔹 **drivecore_cli**: Command-line interface for control and monitoring.

## ⚙️ Getting Started

### Prerequisites

- Install Rust toolchain: `rustup install stable`

### Build & Run

1. Clone the repository:
   ```bash
   git clone https://github.com/<you>/drivecore
   cd Drivecore
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the CLI:
   ```bash
   cargo run --bin drivecore_cli -- --help
   ```

## 🧪 Testing

Run all tests in the workspace:

```bash
cargo test
```

## 🛠 Contribution Guide

- Feature branches
- Code formatting (`rustfmt`)
- Linting (`clippy`)

## 🔐 Safety Notices

- **Always test on closed circuits.**
- **Hardware control = risk of injury.**

## 📄 License

MIT License
