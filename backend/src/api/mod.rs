use serde::{Deserialize, Serialize};
use crate::state::Device;

/// API request/response types

#[derive(Debug, Serialize, Deserialize)]
pub struct StateResponse {
    pub devices: std::collections::HashMap<String, Device>,
    pub vlans: std::collections::HashMap<u16, crate::state::Vlan>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVlanRequest {
    pub vlan_id: u16,
    pub name: String,
    pub subnet: String,
    pub gateway: String,
    pub dhcp_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub mac: String,
    pub name: String,
    pub vlan_id: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetQosRuleRequest {
    pub mac: String,
    pub rate_mbps: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WolRequest {
    pub mac: String,
    pub vlan_id: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

pub mod handlers;
pub mod extensions;
pub mod lan_config;
pub mod cert_handler;
