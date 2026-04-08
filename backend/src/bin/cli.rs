use clap::Parser;

#[derive(Parser)]
#[command(name = "NetCtl")]
#[command(about = "Network Control CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
enum Command {
    /// Create a new VLAN
    VlanCreate {
        #[arg(short, long)]
        id: u16,

        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        subnet: String,
    },

    /// List all VLANs
    VlanList,

    /// Get system state
    State,

    /// Set QoS rule
    QosSet {
        #[arg(short, long)]
        mac: String,

        #[arg(short, long)]
        rate: u32,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::State) => {
            println!("Fetching system state...");
            // TODO: Implement HTTP request to daemon
        }
        Some(Command::VlanCreate { id, name, subnet }) => {
            println!("Creating VLAN {} ({}) with subnet {}", id, name, subnet);
            // TODO: Implement HTTP request to daemon
        }
        Some(Command::VlanList) => {
            println!("Fetching VLAN list...");
            // TODO: Implement HTTP request to daemon
        }
        Some(Command::QosSet { mac, rate }) => {
            println!("Setting QoS rule for {} to {} Mbps", mac, rate);
            // TODO: Implement HTTP request to daemon
        }
        None => {
            println!("NetCtl CLI - Use --help for commands");
        }
    }
}
