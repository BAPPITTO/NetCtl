use crate::error::{Error, Result};
use std::collections::HashMap;

/// QoS (Quality of Service) management for bandwidth control
#[derive(Debug, Default)]
pub struct QosManager {
    rules: HashMap<String, QosRule>,
}

#[derive(Debug, Clone)]
pub struct QosRule {
    pub mac: String,
    pub rate_mbps: u32,
    pub priority: u8,
    pub blocked: bool,
}

impl QosManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set rate limit for a MAC address
    pub fn set_rate_limit(&mut self, mac: String, rate_mbps: u32) -> Result<()> {
        let rule = if rate_mbps == 0 {
            // Rate 0 means block/drop all packets
            QosRule {
                mac: mac.clone(),
                rate_mbps: 0,
                priority: 0,
                blocked: true,
            }
        } else {
            QosRule {
                mac: mac.clone(),
                rate_mbps,
                priority: 5,
                blocked: false,
            }
        };

        self.rules.insert(mac, rule);
        Ok(())
    }

    /// Remove rate limit for a MAC
    pub fn remove_rate_limit(&mut self, mac: &str) -> Result<()> {
        self.rules.remove(mac);
        Ok(())
    }

    /// Get all QoS rules
    pub fn get_rules(&self) -> Vec<QosRule> {
        self.rules.values().cloned().collect()
    }

    /// Get specific rule
    pub fn get_rule(&self, mac: &str) -> Option<QosRule> {
        self.rules.get(mac).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qos_manager() {
        let mut qos = QosManager::new();
        qos.set_rate_limit("aa:bb:cc:dd:ee:ff".to_string(), 100).unwrap();

        let rule = qos.get_rule("aa:bb:cc:dd:ee:ff").unwrap();
        assert_eq!(rule.rate_mbps, 100);
        assert!(!rule.blocked);
    }

    #[test]
    fn test_qos_blocking() {
        let mut qos = QosManager::new();
        qos.set_rate_limit("aa:bb:cc:dd:ee:ff".to_string(), 0).unwrap();

        let rule = qos.get_rule("aa:bb:cc:dd:ee:ff").unwrap();
        assert!(rule.blocked);
    }
}