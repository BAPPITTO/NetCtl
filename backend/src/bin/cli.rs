use clap::Parser;
use reqwest::Client;
use serde_json::Value;

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

const BASE_URL: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();

    match args.command {
        Some(Command::State) => {
            let resp: Value = client
                .get(&format!("{}/api/state", BASE_URL))
                .send()
                .await?
                .json()
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        Some(Command::VlanCreate { id, name, subnet }) => {
            let body = serde_json::json!({
                "vlan_id": id,
                "name": name,
                "subnet": subnet,
                "gateway": "192.168.1.1", // default
                "dhcp_enabled": true
            });
            let resp: Value = client
                .post(&format!("{}/api/vlan", BASE_URL))
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        Some(Command::VlanList) => {
            let resp: Value = client
                .get(&format!("{}/api/state", BASE_URL))
                .send()
                .await?
                .json()
                .await?;
            if let Some(vlans) = resp.get("data").and_then(|d| d.get("vlans")) {
                println!("{}", serde_json::to_string_pretty(vlans)?);
            } else {
                println!("{}", serde_json::to_string_pretty(&resp)?);
            }
        }
        Some(Command::QosSet { mac, rate }) => {
            let body = serde_json::json!({ "mac": mac, "rate_mbps": rate });
            let resp: Value = client
                .post(&format!("{}/api/qos/device/{}", BASE_URL, mac))
                .json(&body)
                .send()
                .await?
                .json()
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        None => {
            println!("NetCtl CLI - Use --help for commands");
        }
    }

    Ok(())
}