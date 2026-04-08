use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::SystemState;
use super::{ApiResponse, CreateVlanRequest, CreateDeviceRequest, SetQosRuleRequest};

pub type SharedState = Arc<RwLock<SystemState>>;

/// Create API router
pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/state", get(get_state))
        .route("/api/interfaces", get(get_interfaces))
        .route("/api/vlan", post(create_vlan))
        .route("/api/vlan/:vlan_id", delete(delete_vlan))
        .route("/api/devices", get(get_devices))
        .route("/api/devices", post(create_device))
        .route("/api/qos/device/:mac", post(set_qos_rule))
        .route("/api/qos/device/:mac", delete(remove_qos_rule))
        .route("/api/qos/devices", get(get_qos_rules))
        .route("/api/metrics/summary", get(get_metrics_summary))
        .route("/api/health", get(health_check))
        .with_state(state)
}

async fn get_state(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    Json(ApiResponse::ok(json!({
        "devices": state.devices,
        "vlans": state.vlans,
        "ipv4_forwarding_enabled": state.ipv4_forwarding_enabled,
        "xdp_attached": state.xdp_attached_interfaces,
        "timestamp": state.updated_at,
    })))
}

async fn get_interfaces() -> impl IntoResponse {
    match crate::network::detect_lan_interfaces() {
        Ok(interfaces) => {
            Json(ApiResponse::ok(json!({ "interfaces": interfaces })))
        }
        Err(e) => {
            Json(ApiResponse::<serde_json::Value>::err(e.to_string()))
        }
    }
}

async fn create_vlan(
    State(state): State<SharedState>,
    Json(req): Json<CreateVlanRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    let vlan = crate::state::Vlan {
        id: req.vlan_id,
        name: req.name,
        subnet: req.subnet,
        gateway: req.gateway,
        dhcp_enabled: req.dhcp_enabled,
        dhcp_start: "".to_string(),
        dhcp_end: "".to_string(),
        interface: format!("eth0.{}", req.vlan_id),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    if let Err(e) = state.add_vlan(vlan.clone()) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<serde_json::Value>::err(e.to_string())),
        )
            .into_response();
    }

    (StatusCode::CREATED, Json(ApiResponse::ok(vlan))).into_response()
}

async fn delete_vlan(
    State(state): State<SharedState>,
    Path(vlan_id): Path<u16>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    if let Err(e) = state.remove_vlan(vlan_id) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<serde_json::Value>::err(e.to_string())),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(ApiResponse::ok(json!({ "vlan_id": vlan_id, "deleted": true }))),
    )
        .into_response()
}

async fn get_devices(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let devices: Vec<_> = state.devices.values().cloned().collect();
    Json(ApiResponse::ok(devices))
}

async fn create_device(
    State(state): State<SharedState>,
    Json(req): Json<CreateDeviceRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;

    let device = crate::state::Device {
        id: uuid::Uuid::new_v4().to_string(),
        mac: req.mac,
        name: req.name,
        vlan_id: req.vlan_id,
        rate_limit_mbps: None,
        blocked: false,
        created_at: chrono::Utc::now().to_rfc3339(),
        last_seen: None,
    };

    if let Err(e) = state.add_device(device.clone()) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<serde_json::Value>::err(e.to_string())),
        )
            .into_response();
    }

    (StatusCode::CREATED, Json(ApiResponse::ok(device))).into_response()
}

async fn set_qos_rule(
    State(state): State<SharedState>,
    Path(mac): Path<String>,
    Json(req): Json<SetQosRuleRequest>,
) -> impl IntoResponse {
    let mut state = state.write().await;
    state.qos_rules.insert(mac.clone(), req.rate_mbps);
    state.update_timestamp();

    Json(ApiResponse::ok(json!({
        "mac": mac,
        "rate_mbps": req.rate_mbps,
    })))
}

async fn remove_qos_rule(
    State(state): State<SharedState>,
    Path(mac): Path<String>,
) -> impl IntoResponse {
    let mut state = state.write().await;
    state.qos_rules.remove(&mac);
    state.update_timestamp();

    Json(ApiResponse::ok(json!({
        "mac": mac,
        "removed": true,
    })))
}

async fn get_qos_rules(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    Json(ApiResponse::ok(state.qos_rules.clone()))
}

async fn get_metrics_summary() -> impl IntoResponse {
    Json(ApiResponse::ok(json!({
        "devices": 0,
        "total_rate_mbps": 0.0,
        "packets_dropped": 0,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::ok(json!({ "status": "healthy" })))
}
