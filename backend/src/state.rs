use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use crate::error::{Error, Result};

/// Represents a network device (e.g., VLAN member or managed host)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: String,
    pub mac: String,
    pub name: String,
    pub vlan_id: Option<u16>,
    pub rate_limit_mbps: Option<u32>,
    pub blocked: bool,
    pub created_at: String,
    pub last_seen: Option<String>,
}

/// VLAN configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vlan {
    pub id: u16,
    pub name: String,
    pub subnet: String,
    pub gateway: String,
    pub dhcp_enabled: bool,
    pub dhcp_start: String,
    pub dhcp_end: String,
    pub interface: String,
    pub created_at: String,
}

/// DHCP scope configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpScope {
    pub vlan_id: u16,
    pub interface: String,
    pub range_start: String,
    pub range_end: String,
    pub lease_time: u32,
}

/// Network operation (reversible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetOp {
    CreateVlan { vlan_id: u16, subnet: String, gateway: String },
    DeleteVlan { vlan_id: u16 },
    ConfigureInterface { interface: String, ip: String, netmask: String },
    StartDhcp { vlan_id: u16 },
    StopDhcp { vlan_id: u16 },
    SetFwdEnable,
    SetFwdDisable,
    SetQosRule { mac: String, rate_mbps: u32 },
    RemoveQosRule { mac: String },
    AttachXdp { interface: String },
    DetachXdp { interface: String },
}

impl NetOp {
    /// Generate reverse operation for rollback
    pub fn reverse(&self) -> Result<NetOp> {
        use NetOp::*;
        match self {
            CreateVlan { vlan_id, .. } => Ok(DeleteVlan { vlan_id: *vlan_id }),
            DeleteVlan { vlan_id } => Err(Error::StateError("Cannot auto-reverse DeleteVlan".into())),
            ConfigureInterface { .. } => Err(Error::StateError("Cannot auto-reverse ConfigureInterface".into())),
            StartDhcp { vlan_id } => Ok(StopDhcp { vlan_id: *vlan_id }),
            StopDhcp { vlan_id } => Ok(StartDhcp { vlan_id: *vlan_id }),
            SetFwdEnable => Ok(SetFwdDisable),
            SetFwdDisable => Ok(SetFwdEnable),
            SetQosRule { mac, .. } => Ok(RemoveQosRule { mac: mac.clone() }),
            RemoveQosRule { mac } => Err(Error::StateError(format!("Cannot auto-reverse RemoveQosRule for {}", mac))),
            AttachXdp { interface } => Ok(DetachXdp { interface: interface.clone() }),
            DetachXdp { interface } => Ok(AttachXdp { interface: interface.clone() }),
        }
    }
}

/// Transaction with ordered operations and rollback stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub operations: Vec<NetOp>,
    pub rollback_stack: Vec<NetOp>,
    pub status: TransactionStatus,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    InProgress,
    Applied,
    RolledBack,
    Failed(String),
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            operations: vec![],
            rollback_stack: vec![],
            status: TransactionStatus::Pending,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn add_operation(&mut self, op: NetOp) { self.operations.push(op); }
    pub fn mark_applied(&mut self) { self.status = TransactionStatus::Applied; }
    pub fn mark_failed(&mut self, reason: String) { self.status = TransactionStatus::Failed(reason); }
    pub fn mark_rolled_back(&mut self) { self.status = TransactionStatus::RolledBack; }
}

/// Complete system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub version: u32,
    pub devices: HashMap<String, Device>,
    pub vlans: HashMap<u16, Vlan>,
    pub dhcp_scopes: HashMap<u16, DhcpScope>,
    pub ipv4_forwarding_enabled: bool,
    pub xdp_attached_interfaces: Vec<String>,
    pub qos_rules: HashMap<String, u32>, // MAC -> rate_mbps
    pub last_transaction_id: Option<String>,
    pub updated_at: String,
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            version: 1,
            devices: HashMap::new(),
            vlans: HashMap::new(),
            dhcp_scopes: HashMap::new(),
            ipv4_forwarding_enabled: false,
            xdp_attached_interfaces: vec![],
            qos_rules: HashMap::new(),
            last_transaction_id: None,
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}

impl SystemState {
    pub fn new() -> Self { Self::default() }

    pub fn add_device(&mut self, device: Device) -> Result<()> {
        self.devices.insert(device.id.clone(), device);
        self.update_timestamp();
        Ok(())
    }

    pub fn add_vlan(&mut self, vlan: Vlan) -> Result<()> {
        self.vlans.insert(vlan.id, vlan);
        self.update_timestamp();
        Ok(())
    }

    pub fn remove_vlan(&mut self, vlan_id: u16) -> Result<()> {
        self.vlans.remove(&vlan_id);
        self.dhcp_scopes.remove(&vlan_id);
        self.update_timestamp();
        Ok(())
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now().to_rfc3339();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new();
        assert_eq!(tx.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_netop_reverse() {
        let op = NetOp::SetFwdEnable;
        let reversed = op.reverse().unwrap();
        assert!(matches!(reversed, NetOp::SetFwdDisable));
    }

    #[test]
    fn test_system_state() {
        let mut state = SystemState::new();
        let device = Device {
            id: "dev1".to_string(),
            mac: "aa:bb:cc:dd:ee:ff".to_string(),
            name: "test-device".to_string(),
            vlan_id: None,
            rate_limit_mbps: None,
            blocked: false,
            created_at: Utc::now().to_rfc3339(),
            last_seen: None,
        };
        state.add_device(device).unwrap();
        assert_eq!(state.devices.len(), 1);
    }
}