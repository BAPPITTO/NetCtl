use crate::error::{Error, Result};

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
    use std::process::Command;
    use regex::Regex;

    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"link/ether\s+([\da-f:]+)")
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    if let Some(caps) = re.captures(&stdout) {
        if let Some(mac) = caps.get(1) {
            return Ok(Some(mac.as_str().to_string()));
        }
    }

    Ok(None)
}

/// Get MTU for interface
pub fn get_interface_mtu(interface: &str) -> Result<Option<u32>> {
    use std::process::Command;
    use regex::Regex;

    let output = Command::new("ip")
        .args(&["link", "show", interface])
        .output()
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"mtu\s+(\d+)")
        .map_err(|e| Error::NetworkError(e.to_string()))?;

    if let Some(caps) = re.captures(&stdout) {
        if let Some(mtu_str) = caps.get(1) {
            if let Ok(mtu) = mtu_str.as_str().parse::<u32>() {
                return Ok(Some(mtu));
            }
        }
    }

    Ok(None)
}
