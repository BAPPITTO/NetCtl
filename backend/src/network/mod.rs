use crate::error::{Error, Result};
use std::process::Command;
use std::net::IpAddr;
use std::str::FromStr;
use regex::Regex;

pub mod interfaces;
pub mod commands;

pub use interfaces::*;
pub use commands::*;

/// Detects WAN interface (primary default route)
pub fn detect_wan_interface() -> Result<String> {
    let output = Command::new("ip")
        .args(&["route", "show", "default"])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"dev\s+(\S+)").map_err(|e| Error::NetworkError(e.to_string()))?;

    if let Some(caps) = re.captures(&stdout) {
        if let Some(iface) = caps.get(1) {
            return Ok(iface.as_str().to_string());
        }
    }

    Err(Error::NetworkError("No default route found".to_string()))
}

/// Detects LAN interfaces (physical NICs, excluding virtual)
pub fn detect_lan_interfaces() -> Result<Vec<String>> {
    let output = Command::new("ip")
        .args(&["link", "show"])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"^\d+:\s+(\w+):").map_err(|e| Error::NetworkError(e.to_string()))?;

    let mut interfaces = Vec::new();
    for line in stdout.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(iface) = caps.get(1) {
                let name = iface.as_str();
                // Filter out virtual interfaces and loopback
                if !name.starts_with("lo") && !name.starts_with("docker") 
                    && !name.starts_with("veth") && !name.starts_with("br-") {
                    interfaces.push(name.to_string());
                }
            }
        }
    }

    Ok(interfaces)
}

/// Get interface IP address
pub fn get_interface_ip(interface: &str) -> Result<Option<String>> {
    let output = Command::new("ip")
        .args(&["addr", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"inet\s+([\d\.]+)")
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    if let Some(caps) = re.captures(&stdout) {
        if let Some(ip) = caps.get(1) {
            return Ok(Some(ip.as_str().to_string()));
        }
    }

    Ok(None)
}

/// Check if interface is up
pub fn is_interface_up(interface: &str) -> Result<bool> {
    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("UP"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_interface_info() {
        // This would require system setup to test properly
        // For now, just verify the functions exist
    }
}
