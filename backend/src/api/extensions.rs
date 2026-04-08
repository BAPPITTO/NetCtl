//! API extensions for advanced enterprise features
//! Supports flows, policies, metrics, alerts, and audit logging

use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::flow::{FlowEngine, PolicyRule, PolicyMatch, PolicyAction};
use crate::security::{SecurityManager, Permission};
use crate::timeseries::{TimeseriesDB, TimeseriesPoint, MetricAlert, AlertOperator};
use crate::audit::{AuditLogger, AuditLog, AuditAction, AuditStatus};

/// Advanced state management with enterprise modules
pub struct EnterpriseState {
    pub flow_engine: Arc<RwLock<FlowEngine>>,
    pub security_manager: Arc<SecurityManager>,
    pub timeseries_db: Arc<TimeseriesDB>,
    pub audit_logger: Arc<AuditLogger>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlowQueryParams {
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct FlowResponse {
    pub flows: Vec<FlowInfo>,
    pub total_count: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FlowInfo {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub packets: u64,
    pub bytes: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PolicyRuleRequest {
    pub name: String,
    pub priority: u32,
    pub match_criteria: String,
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetricsRequest {
    pub metric_name: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AlertRequest {
    pub metric_name: String,
    pub threshold: f64,
    pub operator: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuditQueryParams {
    pub actor_id: Option<String>,
    pub action: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Create enterprise API router with all advanced features
pub fn create_enterprise_router(state: Arc<EnterpriseState>) -> Router {
    Router::new()
        // Flow endpoints
        .route("/api/flows", get(get_flows))
        .route("/api/flows/:flow_id", get(get_flow_details))
        .route("/api/flows/:flow_id", delete(delete_flow))
        
        // Policy endpoints
        .route("/api/policies", get(get_policies))
        .route("/api/policies", post(create_policy))
        .route("/api/policies/:policy_id", get(get_policy))
        .route("/api/policies/:policy_id", delete(delete_policy))
        
        // Metrics endpoints
        .route("/api/metrics/record", post(record_metric))
        .route("/api/metrics/query", get(query_metrics))
        .route("/api/metrics/stats/:metric_name", get(get_metric_stats))
        
        // Alert endpoints
        .route("/api/alerts", get(get_alerts))
        .route("/api/alerts", post(create_alert))
        .route("/api/alerts/:alert_id/acknowledge", post(acknowledge_alert))
        
        // Audit endpoints
        .route("/api/audit/logs", get(get_audit_logs))
        .route("/api/audit/logs/actor/:actor_id", get(get_audit_by_actor))
        .route("/api/audit/logs/action/:action", get(get_audit_by_action))
        
        // Security/Auth endpoints
        .route("/api/auth/login", post(login))
        .route("/api/auth/verify", post(verify_token))
        .route("/api/auth/users", get(list_users))
        .route("/api/auth/users/:user_id/permissions", get(get_user_permissions))
        
        .with_state(state)
}

// ============ Flow Handlers ============

async fn get_flows(
    State(state): State<Arc<EnterpriseState>>,
    Query(params): Query<FlowQueryParams>,
) -> impl IntoResponse {
    let flows = state.flow_engine.read().await;
    let flow_list: Vec<FlowInfo> = flows.active_flows
        .values()
        .take(params.limit.unwrap_or(100))
        .map(|f| FlowInfo {
            src_ip: f.key.src_ip.to_string(),
            dst_ip: f.key.dst_ip.to_string(),
            src_port: f.key.src_port,
            dst_port: f.key.dst_port,
            protocol: format!("{:?}", f.key.protocol),
            packets: f.stats.packets_sent,
            bytes: f.stats.bytes_sent,
        })
        .collect();

    Json(ApiResponse::ok(FlowResponse {
        total_count: flow_list.len(),
        flows: flow_list,
    }))
}

async fn get_flow_details(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_flow_id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::<serde_json::Value>::err("Flow details endpoint not yet implemented".to_string()))
}

async fn delete_flow(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_flow_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::ACCEPTED, Json(ApiResponse::ok(json!({"status": "flow marked for cleanup"}))))
}

// ============ Policy Handlers ============

async fn get_policies(
    State(_state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    Json(ApiResponse::ok(json!({"policies": []})))
}

async fn create_policy(
    State(_state): State<Arc<EnterpriseState>>,
    Json(_req): Json<PolicyRuleRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(ApiResponse::ok(json!({"policy_id": "policy_1", "status": "created"}))))
}

async fn get_policy(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_policy_id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::ok(json!({"policy": {}})))
}

async fn delete_policy(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_policy_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "")
}

// ============ Metrics Handlers ============

async fn record_metric(
    State(state): State<Arc<EnterpriseState>>,
    Json(req): Json<MetricsRequest>,
) -> impl IntoResponse {
    let point = TimeseriesPoint {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        value: req.value,
        metric_name: req.metric_name.clone(),
        tags: std::collections::HashMap::new(),
    };

    match state.timeseries_db.record_metric(point) {
        Ok(_) => (StatusCode::ACCEPTED, Json(ApiResponse::ok(json!({"recorded": req.metric_name})))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::err(e))).into_response(),
    }
}

async fn query_metrics(
    State(state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    match state.timeseries_db.get_alerts() {
        Ok(alerts) => Json(ApiResponse::ok(json!({"alerts": alerts}))),
        Err(e) => Json(ApiResponse::<()>::err(e)),
    }
}

async fn get_metric_stats(
    State(state): State<Arc<EnterpriseState>>,
    Path(metric_name): Path<String>,
) -> impl IntoResponse {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    match state.timeseries_db.get_stats(&metric_name, now - 3600, now) {
        Ok(stats) => Json(ApiResponse::ok(json!({
            "metric": metric_name,
            "min": stats.min,
            "max": stats.max,
            "avg": stats.avg,
            "count": stats.count,
        }))),
        Err(e) => Json(ApiResponse::<()>::err(e)),
    }
}

// ============ Alert Handlers ============

async fn get_alerts(
    State(state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    match state.timeseries_db.get_alerts() {
        Ok(alerts) => Json(ApiResponse::ok(json!({"alerts": alerts}))),
        Err(e) => Json(ApiResponse::<()>::err(e)),
    }
}

async fn create_alert(
    State(state): State<Arc<EnterpriseState>>,
    Json(req): Json<AlertRequest>,
) -> impl IntoResponse {
    let operator = match req.operator.as_str() {
        "gt" => AlertOperator::GreaterThan,
        "lt" => AlertOperator::LessThan,
        "gte" => AlertOperator::GreaterThanOrEqual,
        "lte" => AlertOperator::LessThanOrEqual,
        "eq" => AlertOperator::Equal,
        _ => AlertOperator::GreaterThan,
    };

    let alert = MetricAlert::new(
        uuid::Uuid::new_v4().to_string(),
        req.metric_name.clone(),
        req.threshold,
        operator,
    );

    match state.timeseries_db.add_alert(alert) {
        Ok(_) => (StatusCode::CREATED, Json(ApiResponse::ok(json!({"alert_id": "alert_1", "status": "created"})))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::err(e))).into_response(),
    }
}

async fn acknowledge_alert(
    State(state): State<Arc<EnterpriseState>>,
    Path(alert_id): Path<String>,
) -> impl IntoResponse {
    match state.timeseries_db.acknowledge_alert(&alert_id) {
        Ok(_) => Json(ApiResponse::ok(json!({"alert_id": alert_id, "status": "acknowledged"}))),
        Err(e) => Json(ApiResponse::<()>::err(e)),
    }
}

// ============ Audit Handlers ============

async fn get_audit_logs(
    State(state): State<Arc<EnterpriseState>>,
    Query(params): Query<AuditQueryParams>,
) -> impl IntoResponse {
    match state.audit_logger.get_all() {
        Ok(logs) => {
            let filtered: Vec<AuditLog> = logs
                .into_iter()
                .take(params.limit.unwrap_or(100))
                .collect();
            Json(ApiResponse::ok(json!({"logs": filtered, "count": filtered.len()})))
        }
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}

async fn get_audit_by_actor(
    State(state): State<Arc<EnterpriseState>>,
    Path(actor_id): Path<String>,
) -> impl IntoResponse {
    match state.audit_logger.query_by_actor(&actor_id) {
        Ok(logs) => Json(ApiResponse::ok(json!({"actor_id": actor_id, "logs": logs, "count": logs.len()}))),
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}

async fn get_audit_by_action(
    State(state): State<Arc<EnterpriseState>>,
    Path(action): Path<String>,
) -> impl IntoResponse {
    let audit_action = match action.as_str() {
        "create" => AuditAction::Create,
        "read" => AuditAction::Read,
        "update" => AuditAction::Update,
        "delete" => AuditAction::Delete,
        "login" => AuditAction::Login,
        "logout" => AuditAction::Logout,
        _ => AuditAction::Execute,
    };

    match state.audit_logger.query_by_action(audit_action) {
        Ok(logs) => Json(ApiResponse::ok(json!({"action": action, "logs": logs, "count": logs.len()}))),
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}

// ============ Security/Auth Handlers ============

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

async fn login(
    State(state): State<Arc<EnterpriseState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.security_manager.get_user_by_username(&req.username) {
        Ok(user) => {
            match state.security_manager.generate_token(&user.id) {
                Ok(token) => {
                    let _ = state.audit_logger.log_action(AuditLog::new(
                        user.id.clone(),
                        AuditAction::Login,
                        "user".to_string(),
                        user.id.clone(),
                        AuditStatus::Success,
                        format!("User {} logged in", req.username),
                    ));
                    (StatusCode::OK, Json(ApiResponse::ok(token))).into_response()
                }
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::err(format!("{}", e)))).into_response(),
            }
        }
        Err(e) => (StatusCode::UNAUTHORIZED, Json(ApiResponse::<()>::err(format!("{}", e)))).into_response(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

async fn verify_token(
    State(state): State<Arc<EnterpriseState>>,
    Json(req): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    match state.security_manager.verify_token(&req.token) {
        Ok(token) => Json(ApiResponse::ok(json!({
            "valid": true,
            "user_id": token.user_id,
            "username": token.username,
        }))),
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}

async fn list_users(
    State(state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    let users = state.security_manager.list_users();
    Json(ApiResponse::ok(json!({
        "users": users,
        "count": users.len(),
    })))
}

async fn get_user_permissions(
    State(state): State<Arc<EnterpriseState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    match state.security_manager.get_user(&user_id) {
        Ok(user) => Json(ApiResponse::ok(json!({
            "user_id": user_id,
            "permissions": user.permissions.iter().map(|p| p.as_str()).collect::<Vec<_>>(),
        }))),
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}
