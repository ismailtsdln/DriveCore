use clap::{Parser, Subcommand};
use colored::Colorize;
use drivecore::DriveCore;
use drivecore_common::{ControlCommand, DriveCoreConfig};
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
    /// Analyze CAN Traffic
    Analyze,
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
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    println!("{}", "🚗 DriveCore CLI v0.1.0".bold().cyan());
    println!("{}", "---------------------------".cyan());

    // Load configuration (Using default for now, envision loading from file here)
    let config = DriveCoreConfig::default();
    println!("Loaded Configuration: {:?}", config);

    // In a real scenario, we would choose the vehicle implementation based on config or args
    // For now, let's switch to KiaSoul if user wants? Or stick to Mock but mention Kia logic.
    // Let's use KiaSoul for demonstration to prove the new crate works!
    use drivecore_vehicle::KiaSoul;
    let vehicle = Box::new(KiaSoul::new());
    let core = DriveCore::new(vehicle, config);

    println!("{}", "connecting to vehicle (Kia Soul)...".yellow());
    match core.connect().await {
        Ok(_) => println!("{}", "✔ Connected successfully.".green().bold()),
        Err(e) => {
            eprintln!("{} {}", "✖ Connection failed:".red().bold(), e);
            return Ok(());
        }
    }

    match cli.command {
        Commands::Monitor => {
            println!("{}", "📡 Entering Monitor Mode (Ctrl+C to stop)...".blue());
            // Spawn a heartbeat task to keep watchdog happy
            // let core_clone = core.clone(); // Not cloneable? DriveCore is not Clone naturally?
            // DriveCore implementation:
            // pub struct DriveCore { vehicle: Arc<...>, ... }
            // It is not Clone derived. But inner fields are Arc.
            // We need to implement Clone for DriveCore or wrap it in Arc.
            // Let's assume user just runs monitor. Wait, monitor loop blocks main thread.
            // We need to spawn heartbeat.

            // To fix "clone" issue, let's wrap logic or just send heartbeat in loop.
            loop {
                // Send heartbeat
                core.heartbeat().await;

                match core.get_state().await {
                    Ok(state) => {
                        // Clear previous line (simple ansi)
                        // print!("\x1B[2J\x1B[1;1H");
                        println!(
                            "{} | Speed: {} km/h | Steering: {} | Battery: {}%",
                            "STATUS".bold().white().on_blue(),
                            format!("{:.1}", state.speed_kmh).green(),
                            format!("{:.2}", state.steering_angle).yellow(),
                            state
                                .battery_level
                                .map(|b| format!("{:.1}", b))
                                .unwrap_or("N/A".into())
                                .magenta()
                        );
                    }
                    Err(e) => eprintln!("{} {}", "Error reading state:".red(), e),
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
        Commands::Analyze => {
            println!("{}", "🔍 Analyzing CAN Traffic (Simulated)...".blue());
            println!("{}", "ID   | DATA".dimmed());
            loop {
                core.heartbeat().await;
                // Determine ID and data randomly for simulation
                let id = 0x123;
                let data = vec![0x01, 0x02, 0x03, 0x04];
                println!("{} | {:?}", format!("0x{:X}", id).yellow(), data);
                sleep(Duration::from_millis(500)).await;
            }
        }
        Commands::Control {
            throttle,
            brake,
            steering,
        } => {
            println!("{}", "🎮 Sending Control Command...".purple());
            println!(
                "   ➔ Throttle: {}\n   ➔ Brake:    {}\n   ➔ Steering: {}",
                throttle.to_string().yellow(),
                brake.to_string().red(),
                steering.to_string().blue()
            );

            let cmd = ControlCommand {
                throttle,
                brake,
                steering,
            };
            match core.apply_control(cmd).await {
                Ok(_) => println!("{}", "✔ Command sent successfully.".green()),
                Err(e) => eprintln!("{} {}", "✖ Failed to send command:".red().bold(), e),
            }

            // Read back state to see effect (in mock)
            let state = core.get_state().await?;
            println!(
                "{} Speed: {} km/h",
                "New State:".bold(),
                state.speed_kmh.to_string().cyan()
            );
        }
    }

    core.disconnect().await?;
    println!("{}", "🔌 Disconnected.".dimmed());
    Ok(())
}
