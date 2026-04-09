use crate::error::{Error, Result};
use regex::Regex;
use std::process::Command;

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

    Ok(re
        .captures(&stdout)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .ok_or_else(|| Error::NetworkError("No default route found".to_string()))?)
}

/// Detects LAN interfaces (physical NICs, excluding virtual)
pub fn detect_lan_interfaces() -> Result<Vec<String>> {
    let output = Command::new("ip")
        .args(&["link", "show"])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"^\d+:\s+(\w+):").map_err(|e| Error::NetworkError(e.to_string()))?;

    let interfaces = stdout
        .lines()
        .filter_map(|line| re.captures(line).and_then(|caps| caps.get(1).map(|m| m.as_str())))
        .filter(|name| {
            !name.starts_with("lo")
                && !name.starts_with("docker")
                && !name.starts_with("veth")
                && !name.starts_with("br-")
        })
        .map(String::from)
        .collect();

    Ok(interfaces)
}

/// Get interface IP address
pub fn get_interface_ip(interface: &str) -> Result<Option<String>> {
    let output = Command::new("ip")
        .args(&["addr", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"inet\s+([\d\.]+)").map_err(|e| Error::NetworkError(e.to_string()))?;

    Ok(re.captures(&stdout).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())))
}

/// Check if interface is up
pub fn is_interface_up(interface: &str) -> Result<bool> {
    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    Ok(String::from_utf8_lossy(&output.stdout).contains("UP"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_interface_info() {
        // Exists to ensure functions compile; real testing requires system interfaces
    }
}