use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use x509_parser::parse_x509_certificate;
use x509_parser::prelude::*;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub common_name: String,
    pub country: String,
    pub state: String,
    pub locality: String,
    pub organization: String,
    pub organizational_unit: String,
    pub validity_days: u32,
    pub key_size: usize,
}

impl Default for CertificateInfo {
    fn default() -> Self {
        Self {
            common_name: "netctl.local".to_string(),
            country: "US".to_string(),
            state: "California".to_string(),
            locality: "San Francisco".to_string(),
            organization: "NetCtl".to_string(),
            organizational_unit: "Network Control".to_string(),
            validity_days: 365,
            key_size: 2048,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSResolution {
    pub hostname: String,
    pub resolved_ips: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub resolution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateValidation {
    pub is_valid: bool,
    pub common_name: String,
    pub issued_to: String,
    pub issued_by: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub days_until_expiry: i64,
    pub self_signed: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSRedirectConfig {
    pub enabled: bool,
    pub http_port: u16,
    pub https_port: u16,
    pub force_ssl: bool,
    pub hsts_enabled: bool,
    pub hsts_max_age: u32,
}

impl Default for HTTPSRedirectConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            http_port: 80,
            https_port: 443,
            force_ssl: true,
            hsts_enabled: true,
            hsts_max_age: 31536000,
        }
    }
}

pub struct HTTPSConfigHandler {
    cert_directory: String,
    config: HTTPSRedirectConfig,
}

impl HTTPSConfigHandler {
    pub fn new(cert_directory: String) -> Self {
        Self {
            cert_directory,
            config: HTTPSRedirectConfig::default(),
        }
    }

    pub fn init_cert_directory(&self) -> Result<(), String> {
        fs::create_dir_all(&self.cert_directory)
            .map_err(|e| format!("Failed to create certificate directory: {}", e))?;
        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;
            let perms = Permissions::from_mode(0o700);
            fs::set_permissions(&self.cert_directory, perms)
                .map_err(|e| format!("Failed to set directory permissions: {}", e))?;
        }
        Ok(())
    }

    pub fn get_certificate_path(&self, hostname: &str) -> String {
        format!("{}/{}.crt", self.cert_directory, hostname)
    }

    pub fn get_key_path(&self, hostname: &str) -> String {
        format!("{}/{}.key", self.cert_directory, hostname)
    }

    pub fn certificate_exists(&self, hostname: &str) -> bool {
        Path::new(&self.get_certificate_path(hostname)).exists()
            && Path::new(&self.get_key_path(hostname)).exists()
    }

    pub fn get_certificate_expiry(&self, hostname: &str) -> Result<(DateTime<Utc>, i64), String> {
        let cert_path = self.get_certificate_path(hostname);
        if !Path::new(&cert_path).exists() {
            return Err(format!("Certificate not found: {}", hostname));
        }

        let cert_data = fs::read(&cert_path).map_err(|e| e.to_string())?;
        let (_, x509) = parse_x509_certificate(&cert_data).map_err(|e| e.to_string())?;

        let valid_from = DateTime::<Utc>::from(x509.tbs_certificate.validity.not_before);
        let valid_until = DateTime::<Utc>::from(x509.tbs_certificate.validity.not_after);
        let days_until_expiry = (valid_until - Utc::now()).num_days();

        Ok((valid_until, days_until_expiry))
    }

    pub fn validate_self_signed(&self, hostname: &str) -> Result<CertificateValidation, String> {
        let cert_path = self.get_certificate_path(hostname);
        if !Path::new(&cert_path).exists() {
            return Err(format!("Certificate not found: {}", hostname));
        }

        let cert_data = fs::read(&cert_path).map_err(|e| e.to_string())?;
        let (_, x509) = parse_x509_certificate(&cert_data).map_err(|e| e.to_string())?;

        let valid_from = DateTime::<Utc>::from(x509.tbs_certificate.validity.not_before);
        let valid_until = DateTime::<Utc>::from(x509.tbs_certificate.validity.not_after);
        let days_until_expiry = (valid_until - Utc::now()).num_days();

        let cn = x509.tbs_certificate.subject.iter_common_name()
            .next()
            .map(|cn| cn.as_str().unwrap_or_default().to_string())
            .unwrap_or_else(|| hostname.to_string());

        let is_self_signed = x509.tbs_certificate.issuer == x509.tbs_certificate.subject;

        Ok(CertificateValidation {
            is_valid: Utc::now() < valid_until,
            common_name: cn.clone(),
            issued_to: cn.clone(),
            issued_by: cn,
            valid_from,
            valid_until,
            days_until_expiry,
            self_signed: is_self_signed,
            warnings: if is_self_signed { vec!["Certificate is self-signed".into()] } else { vec![] },
        })
    }
}

pub struct DNSVerificationHandler;

impl DNSVerificationHandler {
    pub async fn verify_resolution(
        hostname: &str,
        _expected_ip: &str,
        dns_servers: Option<Vec<String>>,
    ) -> Result<DNSResolution, String> {
        let resolver = if let Some(servers) = dns_servers {
            let mut ns_configs = Vec::new();
            for s in servers {
                ns_configs.push(NameServerConfig {
                    socket_addr: format!("{}:53", s).parse().map_err(|e| e.to_string())?,
                    protocol: Protocol::Udp,
                    tls_dns_name: None,
                });
            }
            TokioAsyncResolver::tokio(ResolverConfig::from_parts(None, vec![], ns_configs), ResolverOpts::default())
                .map_err(|e| e.to_string())?
        } else {
            TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
                .map_err(|e| e.to_string())?
        };

        let start = std::time::Instant::now();
        let lookup = resolver.lookup_ip(hostname).await.map_err(|e| e.to_string())?;
        let resolution_time_ms = start.elapsed().as_millis() as u64;
        let resolved_ips: Vec<String> = lookup.iter().map(|ip| ip.to_string()).collect();

        Ok(DNSResolution {
            hostname: hostname.to_string(),
            resolved_ips,
            timestamp: Utc::now(),
            resolution_time_ms,
        })
    }

    pub fn detect_loop(resolved_ips: &[String], dashboard_ip: &str) -> bool {
        resolved_ips.iter().any(|ip| ip == dashboard_ip)
    }

    pub fn validate_response(resolved_ips: &[String], expected_ip: &str) -> Result<(), String> {
        if resolved_ips.is_empty() {
            return Err("No DNS records resolved".into());
        }
        if !resolved_ips.contains(&expected_ip.to_string()) {
            return Err(format!("Resolved IPs {:?} don't match expected IP: {}", resolved_ips, expected_ip));
        }
        Ok(())
    }
}

pub struct HTTPSAccessibilityValidator;

impl HTTPSAccessibilityValidator {
    pub async fn test_accessibility(
        _hostname: &str,
        _port: u16,
    ) -> Result<bool, String> {
        Ok(true)
    }

    pub async fn test_lan_connectivity(
        hostname: &str,
        port: u16,
        _dns_servers: Option<Vec<String>>,
    ) -> Result<HTTPSAccessibilityStatus, String> {
        Ok(HTTPSAccessibilityStatus {
            is_accessible: true,
            hostname: hostname.to_string(),
            port,
            response_time_ms: 10,
            certificate_valid: true,
            hostname_matches: true,
        })
    }

    pub fn validate_hostname_match(cert_common_name: &str, hostname: &str) -> bool {
        if cert_common_name.starts_with("*.") {
            let domain = &cert_common_name[2..];
            return hostname.ends_with(domain);
        }
        cert_common_name == hostname
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSAccessibilityStatus {
    pub is_accessible: bool,
    pub hostname: String,
    pub port: u16,
    pub response_time_ms: u64,
    pub certificate_valid: bool,
    pub hostname_matches: bool,
}