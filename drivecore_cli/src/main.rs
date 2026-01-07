use clap::{Parser, Subcommand};
use drivecore::DriveCore;
use drivecore_common::ControlCommand;
use drivecore_firmware::MockVehicle;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "drivecore-cli")]
#[command(about = "DriveCore Command Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor vehicle state
    Monitor,
    /// Send a test control command
    Control {
        #[arg(long, default_value_t = 0.0)]
        throttle: f32,
        #[arg(long, default_value_t = 0.0)]
        brake: f32,
        #[arg(long, default_value_t = 0.0)]
        steering: f32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // In a real scenario, we would choose the vehicle implementation based on config or args
    let vehicle = Box::new(MockVehicle::new());
    let core = DriveCore::new(vehicle);

    core.connect().await?;

    match cli.command {
        Commands::Monitor => {
            println!("Monitoring vehicle state (Ctrl+C to stop)...");
            loop {
                match core.get_state().await {
                    Ok(state) => println!("State: {:?}", state),
                    Err(e) => eprintln!("Error reading state: {}", e),
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
        Commands::Control {
            throttle,
            brake,
            steering,
        } => {
            println!(
                "Sending control - Throttle: {}, Brake: {}, Steering: {}",
                throttle, brake, steering
            );
            let cmd = ControlCommand {
                throttle,
                brake,
                steering,
            };
            match core.apply_control(cmd).await {
                Ok(_) => println!("Command sent successfully."),
                Err(e) => eprintln!("Failed to send command: {}", e),
            }
            // Read back state to see effect (in mock)
            let state = core.get_state().await?;
            println!("Current State: {:?}", state);
        }
    }

    core.disconnect().await?;
    Ok(())
}
