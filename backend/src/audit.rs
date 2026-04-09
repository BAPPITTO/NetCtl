use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Types of audit actions
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    Login,
    Logout,
    Export,
    Configure,
    Execute,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::Create => "create",
            AuditAction::Read => "read",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
            AuditAction::Login => "login",
            AuditAction::Logout => "logout",
            AuditAction::Export => "export",
            AuditAction::Configure => "configure",
            AuditAction::Execute => "execute",
        }
    }
}

/// Status of an audit action
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuditStatus {
    Success,
    Failure,
    Partial,
}

impl AuditStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditStatus::Success => "success",
            AuditStatus::Failure => "failure",
            AuditStatus::Partial => "partial",
        }
    }
}

/// Represents a single audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub actor_id: String,
    pub action: AuditAction,
    pub resource_type: String,
    pub resource_id: String,
    pub status: AuditStatus,
    pub timestamp: String,
    pub details: String,
}

impl AuditLog {
    /// Create a new audit log entry
    pub fn new(
        actor_id: String,
        action: AuditAction,
        resource_type: String,
        resource_id: String,
        status: AuditStatus,
        details: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            actor_id,
            action,
            resource_type,
            resource_id,
            status,
            timestamp: Utc::now().to_rfc3339(),
            details,
        }
    }
}

/// Error types for audit operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditError {
    QueryFailed,
    LogFailed,
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditError::QueryFailed => write!(f, "Query failed"),
            AuditError::LogFailed => write!(f, "Log failed"),
        }
    }
}

/// Thread-safe audit logger with bounded history
#[derive(Clone)]
pub struct AuditLogger {
    logs: Arc<RwLock<VecDeque<AuditLog>>>,
    max_logs: usize,
}

impl AuditLogger {
    /// Create a new audit logger with bounded history
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(VecDeque::new())),
            max_logs,
        }
    }

    /// Log an action to the audit trail
    pub fn log_action(&self, log: AuditLog) -> Result<(), AuditError> {
        let mut logs = self.logs.write().map_err(|_| AuditError::LogFailed)?;
        logs.push_back(log);

        while logs.len() > self.max_logs {
            logs.pop_front();
        }

        Ok(())
    }

    /// Query logs by actor (user)
    pub fn query_by_actor(&self, actor_id: &str) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self.logs.read().map_err(|_| AuditError::QueryFailed)?;
        Ok(logs.iter().filter(|l| l.actor_id == actor_id).cloned().collect())
    }

    /// Query logs by resource
    pub fn query_by_resource(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self.logs.read().map_err(|_| AuditError::QueryFailed)?;
        Ok(logs
            .iter()
            .filter(|l| l.resource_type == resource_type && l.resource_id == resource_id)
            .cloned()
            .collect())
    }

    /// Query logs by action type
    pub fn query_by_action(&self, action: AuditAction) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self.logs.read().map_err(|_| AuditError::QueryFailed)?;
        Ok(logs.iter().filter(|l| l.action == action).cloned().collect())
    }

    /// Query logs by status
    pub fn query_by_status(&self, status: AuditStatus) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self.logs.read().map_err(|_| AuditError::QueryFailed)?;
        Ok(logs.iter().filter(|l| l.status == status).cloned().collect())
    }

    /// Get all audit logs
    pub fn get_all(&self) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self.logs.read().map_err(|_| AuditError::QueryFailed)?;
        Ok(logs.iter().cloned().collect())
    }

    /// Get the number of logs
    pub fn len(&self) -> usize {
        self.logs.read().map(|l| l.len()).unwrap_or(0)
    }

    /// Check if logger is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all logs
    pub fn clear(&self) -> Result<(), AuditError> {
        let mut logs = self.logs.write().map_err(|_| AuditError::LogFailed)?;
        logs.clear();
        Ok(())
    }
}

pub use self::{AuditAction, AuditError, AuditLog, AuditLogger, AuditStatus};