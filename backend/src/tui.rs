/// Terminal User Interface (TUI) for NetCtl interactive setup
/// Compatible with NetCtl backend modules (`state.rs`, `timeseries.rs`)
/// Provides a wizard-like setup flow with validation

use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

/// Setup wizard screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SetupScreen {
    Welcome,
    InterfaceSelection,
    IPConfiguration,
    DNSConfiguration,
    DashboardSetup,
    SecurityReview,
    Summary,
    InstallationComplete,
}

impl fmt::Display for SetupScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupScreen::Welcome => write!(f, "Welcome to NetCtl"),
            SetupScreen::InterfaceSelection => write!(f, "Network Interface Selection"),
            SetupScreen::IPConfiguration => write!(f, "IP Configuration"),
            SetupScreen::DNSConfiguration => write!(f, "DNS Configuration"),
            SetupScreen::DashboardSetup => write!(f, "Dashboard Setup"),
            SetupScreen::SecurityReview => write!(f, "Security Review"),
            SetupScreen::Summary => write!(f, "Configuration Summary"),
            SetupScreen::InstallationComplete => write!(f, "Installation Complete"),
        }
    }
}

/// TUI application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiApp {
    pub current_screen: SetupScreen,
    pub screen_history: Vec<SetupScreen>,
    pub exit: bool,
    pub error_message: Option<String>,
    pub success_message: Option<String>,

    // Configuration data
    pub selected_interface: Option<String>,
    pub interface_list: Vec<String>,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dashboard_hostname: String,
    pub dashboard_port: u16,
    pub enable_https: bool,
    pub admin_username: String,
    pub admin_password: String,
    pub admin_password_confirm: String,
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            current_screen: SetupScreen::Welcome,
            screen_history: Vec::new(),
            exit: false,
            error_message: None,
            success_message: None,
            selected_interface: None,
            interface_list: Vec::new(),
            ip_address: String::new(),
            netmask: String::new(),
            gateway: String::new(),
            dns_primary: String::new(),
            dns_secondary: String::new(),
            dashboard_hostname: "netctl.local".into(),
            dashboard_port: 3001,
            enable_https: true,
            admin_username: String::new(),
            admin_password: String::new(),
            admin_password_confirm: String::new(),
        }
    }

    pub fn next_screen(&mut self) -> Result<(), String> {
        self.screen_history.push(self.current_screen);

        let next = match self.current_screen {
            SetupScreen::Welcome => SetupScreen::InterfaceSelection,
            SetupScreen::InterfaceSelection => {
                if self.selected_interface.is_some() {
                    SetupScreen::IPConfiguration
                } else {
                    return Err("Please select a network interface".into());
                }
            }
            SetupScreen::IPConfiguration => {
                if self.validate_ip_config() {
                    SetupScreen::DNSConfiguration
                } else {
                    return Err("Invalid IP configuration".into());
                }
            }
            SetupScreen::DNSConfiguration => {
                if self.validate_dns_config() {
                    SetupScreen::DashboardSetup
                } else {
                    return Err("Invalid DNS configuration".into());
                }
            }
            SetupScreen::DashboardSetup => {
                if !self.dashboard_hostname.is_empty()
                    && !self.admin_username.is_empty()
                    && !self.admin_password.is_empty()
                    && self.admin_password == self.admin_password_confirm
                {
                    SetupScreen::SecurityReview
                } else {
                    return Err("Invalid dashboard configuration".into());
                }
            }
            SetupScreen::SecurityReview => SetupScreen::Summary,
            SetupScreen::Summary => SetupScreen::InstallationComplete,
            SetupScreen::InstallationComplete => SetupScreen::InstallationComplete,
        };

        self.current_screen = next;
        self.error_message = None;
        Ok(())
    }

    pub fn prev_screen(&mut self) {
        if let Some(prev) = self.screen_history.pop() {
            self.current_screen = prev;
            self.error_message = None;
        }
    }

    fn validate_ip_config(&self) -> bool {
        !self.ip_address.is_empty()
            && !self.netmask.is_empty()
            && !self.gateway.is_empty()
            && is_valid_ipv4(&self.ip_address)
            && is_valid_ipv4(&self.netmask)
            && is_valid_ipv4(&self.gateway)
    }

    fn validate_dns_config(&self) -> bool {
        if self.dns_primary.is_empty() {
            return false;
        }
        is_valid_ipv4(&self.dns_primary)
            && (self.dns_secondary.is_empty() || is_valid_ipv4(&self.dns_secondary))
    }

    pub fn get_config_map(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert(
            "interface".into(),
            self.selected_interface.clone().unwrap_or_default(),
        );
        config.insert("ip_address".into(), self.ip_address.clone());
        config.insert("netmask".into(), self.netmask.clone());
        config.insert("gateway".into(), self.gateway.clone());
        config.insert("dns_primary".into(), self.dns_primary.clone());
        config.insert("dns_secondary".into(), self.dns_secondary.clone());
        config.insert("dashboard_hostname".into(), self.dashboard_hostname.clone());
        config.insert("dashboard_port".into(), self.dashboard_port.to_string());
        config.insert("enable_https".into(), self.enable_https.to_string());
        config.insert("admin_username".into(), self.admin_username.clone());
        config
    }

    pub fn set_interface_list(&mut self, interfaces: Vec<String>) {
        self.interface_list = interfaces;
    }

    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    pub fn is_configured(&self) -> bool {
        self.selected_interface.is_some()
            && !self.ip_address.is_empty()
            && !self.netmask.is_empty()
            && !self.gateway.is_empty()
            && !self.dns_primary.is_empty()
            && !self.dashboard_hostname.is_empty()
            && !self.admin_username.is_empty()
            && !self.admin_password.is_empty()
            && self.admin_password == self.admin_password_confirm
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Deserialize from JSON string
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

fn is_valid_ipv4(ip: &str) -> bool {
    ip.split('.').count() == 4
        && ip.split('.')
            .all(|part| part.parse::<u8>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let mut app = TuiApp::new();
        app.selected_interface = Some("eth0".into());
        app.ip_address = "192.168.1.100".into();

        let json = app.to_json();
        let deserialized = TuiApp::from_json(&json).unwrap();
        assert_eq!(deserialized.selected_interface, Some("eth0".into()));
        assert_eq!(deserialized.ip_address, "192.168.1.100");
    }
}