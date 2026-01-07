use clap::{Parser, Subcommand};
use colored::Colorize;
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

    println!("{}", "🚗 DriveCore CLI v0.1.0".bold().cyan());
    println!("{}", "---------------------------".cyan());

    // In a real scenario, we would choose the vehicle implementation based on config or args
    let vehicle = Box::new(MockVehicle::new());
    let core = DriveCore::new(vehicle);

    println!("{}", "connecting to vehicle...".yellow());
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
            loop {
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
