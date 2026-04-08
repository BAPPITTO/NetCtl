/// LAN Dashboard Configuration API
/// Handles hostname, DNS verification, HTTPS setup, and accessibility

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;

/// Dashboard configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub hostname: String,
    pub port: u16,
    pub enable_https: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
    pub dns_domain: String,
    pub enable_dns_verification: bool,
    pub local_ip_address: String,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            hostname: "netctl.local".to_string(),
            port: 443,
            enable_https: true,
            certificate_path: None,
            key_path: None,
            dns_domain: "local".to_string(),
            enable_dns_verification: true,
            local_ip_address: "127.0.0.1".to_string(),
        }
    }
}

/// Request to configure dashboard settings
#[derive(Debug, Deserialize)]
pub struct ConfigureDashboardRequest {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub enable_https: Option<bool>,
    pub dns_domain: Option<String>,
    pub local_ip_address: String,
}

/// Response from dashboard configuration
#[derive(Debug, Serialize)]
pub struct ConfigureDashboardResponse {
    pub success: bool,
    pub hostname: String,
    pub port: u16,
    pub url: String,
    pub certificate_generated: bool,
    pub message: String,
}

/// DNS verification result
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum DNSVerificationStatus {
    Valid,
    Invalid,
    Loopback,
    Unreachable,
    Misconfigured,
}

impl std::fmt::Display for DNSVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid => write!(f, "Valid"),
            Self::Invalid => write!(f, "Invalid"),
            Self::Loopback => write!(f, "Loopback"),
            Self::Unreachable => write!(f, "Unreachable"),
            Self::Misconfigured => write!(f, "Misconfigured"),
        }
    }
}

/// DNS verification request
#[derive(Debug, Deserialize)]
pub struct VerifyDNSRequest {
    pub hostname: String,
    pub dns_servers: Option<Vec<String>>,
    pub expected_ip: String,
}

/// DNS verification response
#[derive(Debug, Serialize)]
pub struct VerifyDNSResponse {
    pub hostname: String,
    pub status: DNSVerificationStatus,
    pub resolved_ip: Option<String>,
    pub expected_ip: String,
    pub loop_detected: bool,
    pub message: String,
}

/// HTTPS certificate configuration request
#[derive(Debug, Deserialize)]
pub struct ConfigureCertificateRequest {
    pub use_self_signed: bool,
    pub common_name: String,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub organization: Option<String>,
    pub validity_days: Option<u32>,
}

/// HTTPS certificate configuration response
#[derive(Debug, Serialize)]
pub struct ConfigureCertificateResponse {
    pub success: bool,
    pub certificate_path: String,
    pub key_path: String,
    pub valid_from: String,
    pub valid_until: String,
    pub common_name: String,
    pub message: String,
}

/// DNS configuration helper
#[derive(Debug, Clone)]
pub struct DNSConfiguration {
    pub primary_dns: String,
    pub secondary_dns: Option<String>,
    pub search_domains: Vec<String>,
}

impl DNSConfiguration {
    /// Create a new DNS configuration
    pub fn new(primary_dns: String) -> Self {
        Self {
            primary_dns,
            secondary_dns: None,
            search_domains: vec!["local".to_string()],
        }
    }

    /// Add a secondary DNS server
    pub fn with_secondary(mut self, secondary_dns: String) -> Self {
        self.secondary_dns = Some(secondary_dns);
        self
    }

    /// Add search domains
    pub fn with_search_domains(mut self, domains: Vec<String>) -> Self {
        self.search_domains = domains;
        self
    }

    /// Verify DNS is valid
    pub fn validate(&self) -> Result<(), String> {
        if !is_valid_ipv4(&self.primary_dns) {
            return Err("Invalid primary DNS server IP address".to_string());
        }
        if let Some(ref secondary) = self.secondary_dns {
            if !is_valid_ipv4(secondary) {
                return Err("Invalid secondary DNS server IP address".to_string());
            }
        }
        Ok(())
    }
}

/// Verify hostname format
pub fn validate_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }

    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.is_empty() {
        return false;
    }

    for part in parts {
        if part.is_empty() || part.len() > 63 {
            return false;
        }
        if !part.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
    }

    true
}

/// Validate IPv4 address
pub fn is_valid_ipv4(ip: &str) -> bool {
    IpAddr::from_str(ip)
        .map(|addr| matches!(addr, IpAddr::V4(_)))
        .unwrap_or(false)
}

/// Detect DNS loop (hostname resolves to itself)
pub fn detect_dns_loop(resolved_ip: &str, expected_ip: &str) -> bool {
    resolved_ip == expected_ip
}

/// Generate self-signed certificate paths
pub fn generate_cert_paths(hostname: &str) -> (String, String) {
    let cert_dir = format!("/etc/netctl/certificates");
    let cert_path = format!("{}/{}.crt", cert_dir, hostname);
    let key_path = format!("{}/{}.key", cert_dir, hostname);
    (cert_path, key_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dashboard_config() {
        let config = DashboardConfig::default();
        assert_eq!(config.hostname, "netctl.local");
        assert_eq!(config.port, 443);
        assert!(config.enable_https);
    }

    #[test]
    fn test_validate_hostname_valid() {
        assert!(validate_hostname("netctl.local"));
        assert!(validate_hostname("my-dashboard"));
        assert!(validate_hostname("dashboard123"));
        assert!(validate_hostname("a"));
    }

    #[test]
    fn test_validate_hostname_invalid() {
        assert!(!validate_hostname(""));
        assert!(!validate_hostname("-invalid"));
        assert!(!validate_hostname("invalid-"));
        assert!(!validate_hostname("in--valid"));
        assert!(!validate_hostname("a".repeat(64))); // > 63 chars per label
    }

    #[test]
    fn test_validate_ipv4() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("8.8.8.8"));
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(is_valid_ipv4("255.255.255.255"));
        assert!(!is_valid_ipv4("256.1.1.1"));
        assert!(!is_valid_ipv4("192.168.1"));
        assert!(!is_valid_ipv4("invalid"));
        assert!(!is_valid_ipv4("::1")); // IPv6
    }

    #[test]
    fn test_dns_loop_detection() {
        assert!(detect_dns_loop("192.168.1.100", "192.168.1.100"));
        assert!(!detect_dns_loop("192.168.1.100", "192.168.1.101"));
    }

    #[test]
    fn test_dns_configuration_validation() {
        let config = DNSConfiguration::new("8.8.8.8".to_string());
        assert!(config.validate().is_ok());

        let config = DNSConfiguration::new("invalid".to_string());
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_dns_configuration_with_secondary() {
        let config = DNSConfiguration::new("8.8.8.8".to_string())
            .with_secondary("8.8.4.4".to_string());

        assert_eq!(config.primary_dns, "8.8.8.8");
        assert_eq!(config.secondary_dns, Some("8.8.4.4".to_string()));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_certificate_paths() {
        let (cert_path, key_path) = generate_cert_paths("netctl");
        assert!(cert_path.contains("netctl"));
        assert!(key_path.contains("netctl"));
        assert!(cert_path.ends_with(".crt"));
        assert!(key_path.ends_with(".key"));
    }

    #[test]
    fn test_dns_verification_status_display() {
        assert_eq!(DNSVerificationStatus::Valid.to_string(), "Valid");
        assert_eq!(DNSVerificationStatus::Invalid.to_string(), "Invalid");
        assert_eq!(DNSVerificationStatus::Loopback.to_string(), "Loopback");
    }

    #[test]
    fn test_configure_dashboard_response_serialization() {
        let response = ConfigureDashboardResponse {
            success: true,
            hostname: "netctl.local".to_string(),
            port: 443,
            url: "https://netctl.local".to_string(),
            certificate_generated: true,
            message: "Configuration applied".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("netctl.local"));
        assert!(json.contains("true"));
    }
}
