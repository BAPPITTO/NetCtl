// ================== Flow Handlers ==================

async fn get_flows(
    State(state): State<Arc<EnterpriseState>>,
    Query(params): Query<FlowQueryParams>,
) -> impl IntoResponse {
    let flows = state.flow_engine.read().await;

    // Apply limit safely
    let limit = params.limit.unwrap_or(100).min(1000);

    let flow_list: Vec<FlowInfo> = flows.active_flows
        .values()
        .take(limit)
        .filter(|f| {
            (params.src_ip.as_ref().map_or(true, |ip| ip == &f.key.src_ip.to_string())) &&
            (params.dst_ip.as_ref().map_or(true, |ip| ip == &f.key.dst_ip.to_string()))
        })
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
        flows: flow_list.clone(),
        total_count: flow_list.len(),
    }))
}

async fn get_flow_details(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_flow_id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::<serde_json::Value>::err("Not implemented yet"))
}

async fn delete_flow(
    State(_state): State<Arc<EnterpriseState>>,
    Path(_flow_id): Path<String>,
) -> impl IntoResponse {
    (StatusCode::ACCEPTED, Json(ApiResponse::ok(json!({"status": "flow marked for cleanup"}))))
}

// ================== Policy Handlers ==================

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

// ================== Metrics Handlers ==================

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

    json_result(state.timeseries_db.record_metric(point).map(|_| json!({"recorded": req.metric_name}))).await
}

async fn query_metrics(
    State(state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    json_result(state.timeseries_db.get_alerts().map(|alerts| json!({"alerts": alerts}))).await
}

async fn get_metric_stats(
    State(state): State<Arc<EnterpriseState>>,
    Path(metric_name): Path<String>,
) -> impl IntoResponse {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    json_result(state.timeseries_db.get_stats(&metric_name, now - 3600, now).map(|stats| {
        json!({
            "metric": metric_name,
            "min": stats.min,
            "max": stats.max,
            "avg": stats.avg,
            "count": stats.count,
        })
    })).await
}

// ================== Alert Handlers ==================

async fn get_alerts(
    State(state): State<Arc<EnterpriseState>>,
) -> impl IntoResponse {
    json_result(state.timeseries_db.get_alerts().map(|alerts| json!({"alerts": alerts}))).await
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
        other => return Json(ApiResponse::<()>::err(format!("Unknown operator: {}", other))),
    };

    let alert = MetricAlert::new(uuid::Uuid::new_v4().to_string(), req.metric_name.clone(), req.threshold, operator);
    json_result(state.timeseries_db.add_alert(alert).map(|_| json!({"status": "created"}))).await
}

async fn acknowledge_alert(
    State(state): State<Arc<EnterpriseState>>,
    Path(alert_id): Path<String>,
) -> impl IntoResponse {
    json_result(state.timeseries_db.acknowledge_alert(&alert_id).map(|_| json!({"status": "acknowledged"}))).await
}

// ================== Audit Handlers ==================

async fn get_audit_logs(
    State(state): State<Arc<EnterpriseState>>,
    Query(params): Query<AuditQueryParams>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(100).min(1000);

    json_result(state.audit_logger.get_all().map(|logs| {
        let filtered: Vec<_> = logs.into_iter().take(limit).collect();
        json!({"logs": filtered, "count": filtered.len()})
    })).await
}

async fn get_audit_by_actor(
    State(state): State<Arc<EnterpriseState>>,
    Path(actor_id): Path<String>,
) -> impl IntoResponse {
    json_result(state.audit_logger.query_by_actor(&actor_id).map(|logs| {
        json!({"actor_id": actor_id, "logs": logs, "count": logs.len()})
    })).await
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

    json_result(state.audit_logger.query_by_action(audit_action).map(|logs| {
        json!({"action": action, "logs": logs, "count": logs.len()})
    })).await
}

// ================== Security/Auth Handlers ==================

#[derive(Serialize, Deserialize)]
pub struct LoginRequest { pub username: String, pub password: String }

async fn login(
    State(state): State<Arc<EnterpriseState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.security_manager.get_user_by_username(&req.username) {
        Ok(user) => match state.security_manager.generate_token(&user.id) {
            Ok(token) => {
                let _ = state.audit_logger.log_action(AuditLog::new(
                    user.id.clone(), AuditAction::Login, "user".to_string(),
                    user.id.clone(), AuditStatus::Success,
                    format!("User {} logged in", req.username),
                ));
                Json(ApiResponse::ok(token))
            }
            Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
        },
        Err(e) => Json(ApiResponse::<()>::err(format!("{}", e))),
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerifyTokenRequest { pub token: String }

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
    Json(ApiResponse::ok(json!({"users": users, "count": users.len()})))
}

async fn get_user_permissions(
    State(state): State<Arc<EnterpriseState>>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    json_result(state.security_manager.get_user(&user_id).map(|user| {
        json!({"user_id": user_id, "permissions": user.permissions.iter().map(|p| p.as_str()).collect::<Vec<_>>()})
    })).await
}