use crate::error::{Error, Result};
use regex::Regex;
use std::process::Command;

/// Interface information
#[derive(Debug, Clone)]
pub struct InterfaceInfo {
    pub name: String,
    pub ip: Option<String>,
    pub netmask: Option<String>,
    pub mac: Option<String>,
    pub mtu: Option<u32>,
    pub is_up: bool,
}

/// Get MAC address for interface
pub fn get_interface_mac(interface: &str) -> Result<Option<String>> {
    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"link/ether\s+([\da-f:]+)")
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    Ok(re.captures(&stdout).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())))
}

/// Get MTU for interface
pub fn get_interface_mtu(interface: &str) -> Result<Option<u32>> {
    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"mtu\s+(\d+)")
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    Ok(re
        .captures(&stdout)
        .and_then(|caps| caps.get(1))
        .and_then(|mtu_str| mtu_str.as_str().parse::<u32>().ok()))
}