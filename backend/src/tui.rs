/// Terminal User Interface (TUI) for NetCtl interactive setup
/// Uses Ratatui framework with Matrix/cyberpunk styling

use std::collections::HashMap;
use std::fmt;

/// Setup wizard screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
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
    /// Create a new TUI application instance
    pub fn new() -> Self {
        Self {
            current_screen: SetupScreen::Welcome,
            screen_history: vec![],
            exit: false,
            error_message: None,
            success_message: None,
            selected_interface: None,
            interface_list: vec![],
            ip_address: String::new(),
            netmask: String::new(),
            gateway: String::new(),
            dns_primary: String::new(),
            dns_secondary: String::new(),
            dashboard_hostname: String::from("netctl.local"),
            dashboard_port: 3001,
            enable_https: true,
            admin_username: String::new(),
            admin_password: String::new(),
            admin_password_confirm: String::new(),
        }
    }

    /// Move to next screen in the wizard
    pub fn next_screen(&mut self) -> Result<(), String> {
        self.screen_history.push(self.current_screen);
        
        let next = match self.current_screen {
            SetupScreen::Welcome => SetupScreen::InterfaceSelection,
            SetupScreen::InterfaceSelection => {
                if self.selected_interface.is_some() {
                    SetupScreen::IPConfiguration
                } else {
                    return Err("Please select a network interface".to_string());
                }
            }
            SetupScreen::IPConfiguration => {
                if self.validate_ip_config() {
                    SetupScreen::DNSConfiguration
                } else {
                    return Err("Invalid IP configuration".to_string());
                }
            }
            SetupScreen::DNSConfiguration => {
                if self.validate_dns_config() {
                    SetupScreen::DashboardSetup
                } else {
                    return Err("Invalid DNS configuration".to_string());
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
                    return Err("Invalid dashboard configuration".to_string());
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

    /// Go back to previous screen
    pub fn prev_screen(&mut self) {
        if !self.screen_history.is_empty() {
            self.current_screen = self.screen_history.pop().unwrap();
            self.error_message = None;
        }
    }

    /// Validate IP configuration
    fn validate_ip_config(&self) -> bool {
        !self.ip_address.is_empty()
            && !self.netmask.is_empty()
            && !self.gateway.is_empty()
            && is_valid_ipv4(&self.ip_address)
            && is_valid_ipv4(&self.netmask)
            && is_valid_ipv4(&self.gateway)
    }

    /// Validate DNS configuration
    fn validate_dns_config(&self) -> bool {
        if self.dns_primary.is_empty() {
            return false;
        }
        is_valid_ipv4(&self.dns_primary)
            && (self.dns_secondary.is_empty() || is_valid_ipv4(&self.dns_secondary))
    }

    /// Get configuration as JSON-serializable HashMap
    pub fn get_config_map(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("interface".to_string(), self.selected_interface.clone().unwrap_or_default());
        config.insert("ip_address".to_string(), self.ip_address.clone());
        config.insert("netmask".to_string(), self.netmask.clone());
        config.insert("gateway".to_string(), self.gateway.clone());
        config.insert("dns_primary".to_string(), self.dns_primary.clone());
        config.insert("dns_secondary".to_string(), self.dns_secondary.clone());
        config.insert("dashboard_hostname".to_string(), self.dashboard_hostname.clone());
        config.insert("dashboard_port".to_string(), self.dashboard_port.to_string());
        config.insert("enable_https".to_string(), self.enable_https.to_string());
        config.insert("admin_username".to_string(), self.admin_username.clone());
        config
    }

    /// Set interface list from system detection
    pub fn set_interface_list(&mut self, interfaces: Vec<String>) {
        self.interface_list = interfaces;
    }

    /// Clear messages after display timeout
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    /// Check if all required fields are filled
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
}

/// Validate IPv4 address format
fn is_valid_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|part| {
        part.parse::<u8>().is_ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = TuiApp::new();
        assert_eq!(app.current_screen, SetupScreen::Welcome);
        assert!(!app.exit);
        assert_eq!(app.dashboard_port, 3001);
    }

    #[test]
    fn test_screen_navigation() {
        let mut app = TuiApp::new();
        app.selected_interface = Some("eth0".to_string());
        assert!(app.next_screen().is_ok());
        assert_eq!(app.current_screen, SetupScreen::InterfaceSelection);
    }

    #[test]
    fn test_interface_validation() {
        let mut app = TuiApp::new();
        let result = app.next_screen();
        assert!(result.is_err());
    }

    #[test]
    fn test_ip_configuration_validation() {
        let app = TuiApp::new();
        assert!(!app.validate_ip_config());

        let mut app = TuiApp::new();
        app.ip_address = "192.168.1.100".to_string();
        app.netmask = "255.255.255.0".to_string();
        app.gateway = "192.168.1.1".to_string();
        assert!(app.validate_ip_config());
    }

    #[test]
    fn test_dns_configuration_validation() {
        let mut app = TuiApp::new();
        assert!(!app.validate_dns_config());

        app.dns_primary = "8.8.8.8".to_string();
        assert!(app.validate_dns_config());

        app.dns_secondary = "8.8.4.4".to_string();
        assert!(app.validate_dns_config());
    }

    #[test]
    fn test_ipv4_validation() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("255.255.255.255"));
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(!is_valid_ipv4("256.1.1.1"));
        assert!(!is_valid_ipv4("192.168.1"));
        assert!(!is_valid_ipv4("invalid"));
    }

    #[test]
    fn test_password_mismatch() {
        let mut app = TuiApp::new();
        app.admin_password = "password123".to_string();
        app.admin_password_confirm = "password456".to_string();
        assert!(!app.is_configured());
    }

    #[test]
    fn test_config_map_generation() {
        let mut app = TuiApp::new();
        app.selected_interface = Some("eth0".to_string());
        app.ip_address = "192.168.1.100".to_string();
        app.dashboard_hostname = "netctl.local".to_string();

        let config = app.get_config_map();
        assert_eq!(config.get("interface"), Some(&"eth0".to_string()));
        assert_eq!(config.get("ip_address"), Some(&"192.168.1.100".to_string()));
    }

    #[test]
    fn test_goback_to_previous_screen() {
        let mut app = TuiApp::new();
        app.selected_interface = Some("eth0".to_string());
        app.next_screen().ok();
        assert_eq!(app.current_screen, SetupScreen::InterfaceSelection);
        app.prev_screen();
        assert_eq!(app.current_screen, SetupScreen::Welcome);
    }

    #[test]
    fn test_error_message_handling() {
        let mut app = TuiApp::new();
        app.error_message = Some("Test error".to_string());
        app.success_message = Some("Test success".to_string());
        app.clear_messages();
        assert!(app.error_message.is_none());
        assert!(app.success_message.is_none());
    }

    #[test]
    fn test_screen_display() {
        assert_eq!(
            SetupScreen::Welcome.to_string(),
            "Welcome to NetCtl"
        );
        assert_eq!(
            SetupScreen::IPConfiguration.to_string(),
            "IP Configuration"
        );
    }
}
