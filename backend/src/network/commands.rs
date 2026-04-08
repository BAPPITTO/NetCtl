use crate::error::{Error, Result};
use std::process::Command;

/// Execute a privileged command safely
pub fn run_privileged_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| Error::SystemCommand(format!("{}: {}", cmd, e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::SystemCommand(format!("{} failed: {}", cmd, stderr)));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Create a VLAN interface
pub fn create_vlan(base_interface: &str, vlan_id: u16) -> Result<()> {
    let vlan_interface = format!("{}.{}", base_interface, vlan_id);
    
    run_privileged_cmd("ip", &[
        "link", "add", 
        "link", base_interface, 
        "name", &vlan_interface,
        "type", "vlan", 
        "id", &vlan_id.to_string()
    ])?;

    run_privileged_cmd("ip", &["link", "set", &vlan_interface, "up"])?;

    Ok(())
}

/// Delete a VLAN interface
pub fn delete_vlan(base_interface: &str, vlan_id: u16) -> Result<()> {
    let vlan_interface = format!("{}.{}", base_interface, vlan_id);
    
    run_privileged_cmd("ip", &["link", "set", &vlan_interface, "down"])?;
    run_privileged_cmd("ip", &["link", "del", &vlan_interface])?;

    Ok(())
}

/// Set IP address on interface
pub fn set_ip_address(interface: &str, ip: &str, netmask: &str) -> Result<()> {
    let cidr = format!("{}/{}", ip, netmask_to_cidr(netmask)?);
    
    run_privileged_cmd("ip", &[
        "addr", "add", &cidr, "dev", interface
    ])?;

    Ok(())
}

/// Remove IP address from interface
pub fn remove_ip_address(interface: &str, ip: &str, netmask: &str) -> Result<()> {
    let cidr = format!("{}/{}", ip, netmask_to_cidr(netmask)?);
    
    run_privileged_cmd("ip", &[
        "addr", "del", &cidr, "dev", interface
    ])?;

    Ok(())
}

/// Enable IPv4 forwarding
pub fn enable_ipv4_forwarding() -> Result<()> {
    run_privileged_cmd("sysctl", &["-w", "net.ipv4.ip_forward=1"])?;
    Ok(())
}

/// Disable IPv4 forwarding
pub fn disable_ipv4_forwarding() -> Result<()> {
    run_privileged_cmd("sysctl", &["-w", "net.ipv4.ip_forward=0"])?;
    Ok(())
}

/// Convert netmask to CIDR notation
fn netmask_to_cidr(netmask: &str) -> Result<String> {
    let parts: Vec<&str> = netmask.split('.').collect();
    if parts.len() != 4 {
        return Err(Error::ConfigError("Invalid netmask format".to_string()));
    }

    let mut cidr = 0;
    for part in parts {
        let octet: u8 = part.parse()
            .map_err(|_| Error::ConfigError("Invalid netmask octet".to_string()))?;
        cidr += octet.count_ones();
    }

    Ok(cidr.to_string())
}

/// Generate DHCP config for dnsmasq
pub fn generate_dhcp_config(vlan_id: u16, interface: &str, range_start: &str, range_end: &str, lease_time: u32) -> Result<String> {
    Ok(format!(
        r#"# DHCP configuration for VLAN {}
dhcp-range={},{}},{}
interface={}
bind-interfaces
dhcp-lease-max=100
dhcp-option-force={},router
"#,
        vlan_id,
        range_start,
        range_end,
        lease_time,
        interface,
        vlan_id,
    ))
}

/// Start dnsmasq service
pub fn start_dhcp_service() -> Result<()> {
    run_privileged_cmd("systemctl", &["start", "dnsmasq"])?;
    Ok(())
}

/// Stop dnsmasq service
pub fn stop_dhcp_service() -> Result<()> {
    run_privileged_cmd("systemctl", &["stop", "dnsmasq"])?;
    Ok(())
}

/// Restart dnsmasq service
pub fn restart_dhcp_service() -> Result<()> {
    run_privileged_cmd("systemctl", &["restart", "dnsmasq"])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netmask_to_cidr() {
        assert_eq!(netmask_to_cidr("255.255.255.0").unwrap(), "24");
        assert_eq!(netmask_to_cidr("255.255.0.0").unwrap(), "16");
        assert_eq!(netmask_to_cidr("255.0.0.0").unwrap(), "8");
    }

    #[test]
    fn test_generate_dhcp_config() {
        let config = generate_dhcp_config(10, "eth0.10", "192.168.10.100", "192.168.10.200", 3600).unwrap();
        assert!(config.contains("192.168.10.100"));
        assert!(config.contains("dhcp-range"));
    }
}
