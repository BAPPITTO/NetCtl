//! Comprehensive audit logging system for compliance and forensics
//! Tracks all system actions with full context and status

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
        AuditLog {
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
pub struct AuditLogger {
    logs: Arc<RwLock<VecDeque<AuditLog>>>,
    max_logs: usize,
}

impl AuditLogger {
    /// Create a new audit logger with bounded history
    pub fn new(max_logs: usize) -> Self {
        AuditLogger {
            logs: Arc::new(RwLock::new(VecDeque::new())),
            max_logs,
        }
    }

    /// Log an action to the audit trail
    pub fn log_action(&self, log: AuditLog) -> Result<(), AuditError> {
        let mut logs = self
            .logs
            .write()
            .map_err(|_| AuditError::LogFailed)?;

        logs.push_back(log);

        // Keep only the most recent max_logs entries
        while logs.len() > self.max_logs {
            logs.pop_front();
        }

        Ok(())
    }

    /// Query logs by actor (user)
    pub fn query_by_actor(&self, actor_id: &str) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self
            .logs
            .read()
            .map_err(|_| AuditError::QueryFailed)?;

        let filtered: Vec<AuditLog> = logs
            .iter()
            .filter(|log| log.actor_id == actor_id)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Query logs by resource
    pub fn query_by_resource(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self
            .logs
            .read()
            .map_err(|_| AuditError::QueryFailed)?;

        let filtered: Vec<AuditLog> = logs
            .iter()
            .filter(|log| log.resource_type == resource_type && log.resource_id == resource_id)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Query logs by action type
    pub fn query_by_action(&self, action: AuditAction) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self
            .logs
            .read()
            .map_err(|_| AuditError::QueryFailed)?;

        let filtered: Vec<AuditLog> = logs
            .iter()
            .filter(|log| log.action == action)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Query logs by status
    pub fn query_by_status(&self, status: AuditStatus) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self
            .logs
            .read()
            .map_err(|_| AuditError::QueryFailed)?;

        let filtered: Vec<AuditLog> = logs
            .iter()
            .filter(|log| log.status == status)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Get all audit logs
    pub fn get_all(&self) -> Result<Vec<AuditLog>, AuditError> {
        let logs = self
            .logs
            .read()
            .map_err(|_| AuditError::QueryFailed)?;

        Ok(logs.iter().cloned().collect())
    }

    /// Get the number of logs
    pub fn len(&self) -> usize {
        self.logs
            .read()
            .map(|logs| logs.len())
            .unwrap_or(0)
    }

    /// Check if logger is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all logs
    pub fn clear(&self) -> Result<(), AuditError> {
        let mut logs = self
            .logs
            .write()
            .map_err(|_| AuditError::LogFailed)?;

        logs.clear();
        Ok(())
    }
}

impl Clone for AuditLogger {
    fn clone(&self) -> Self {
        AuditLogger {
            logs: Arc::clone(&self.logs),
            max_logs: self.max_logs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_creation() {
        let log = AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created new device".to_string(),
        );

        assert_eq!(log.actor_id, "user1");
        assert_eq!(log.action, AuditAction::Create);
        assert_eq!(log.status, AuditStatus::Success);
    }

    #[test]
    fn test_audit_logger_creation() {
        let logger = AuditLogger::new(100);
        assert_eq!(logger.len(), 0);
        assert!(logger.is_empty());
    }

    #[test]
    fn test_log_action() {
        let logger = AuditLogger::new(100);
        let log = AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created device".to_string(),
        );

        let result = logger.log_action(log);
        assert!(result.is_ok());
        assert_eq!(logger.len(), 1);
    }

    #[test]
    fn test_query_by_actor() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user2".to_string(),
            AuditAction::Update,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Updated".to_string(),
        )).unwrap();

        let user1_logs = logger.query_by_actor("user1").unwrap();
        assert_eq!(user1_logs.len(), 1);
        assert_eq!(user1_logs[0].actor_id, "user1");
    }

    #[test]
    fn test_query_by_resource() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Update,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Updated".to_string(),
        )).unwrap();

        let dev1_logs = logger.query_by_resource("device", "dev1").unwrap();
        assert_eq!(dev1_logs.len(), 2);
    }

    #[test]
    fn test_query_by_action() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev2".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Update,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Updated".to_string(),
        )).unwrap();

        let create_logs = logger.query_by_action(AuditAction::Create).unwrap();
        assert_eq!(create_logs.len(), 2);
    }

    #[test]
    fn test_query_by_status() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Update,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Failure,
            "Failed to update".to_string(),
        )).unwrap();

        let success_logs = logger.query_by_status(AuditStatus::Success).unwrap();
        assert_eq!(success_logs.len(), 1);

        let failure_logs = logger.query_by_status(AuditStatus::Failure).unwrap();
        assert_eq!(failure_logs.len(), 1);
    }

    #[test]
    fn test_get_all() {
        let logger = AuditLogger::new(100);
        for i in 0..5 {
            logger.log_action(AuditLog::new(
                format!("user{}", i),
                AuditAction::Create,
                "device".to_string(),
                format!("dev{}", i),
                AuditStatus::Success,
                "Created".to_string(),
            )).unwrap();
        }

        let all_logs = logger.get_all().unwrap();
        assert_eq!(all_logs.len(), 5);
    }

    #[test]
    fn test_clear() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        assert_eq!(logger.len(), 1);
        logger.clear().unwrap();
        assert_eq!(logger.len(), 0);
    }

    #[test]
    fn test_bounded_history() {
        let logger = AuditLogger::new(5);
        for i in 0..10 {
            logger.log_action(AuditLog::new(
                "user1".to_string(),
                AuditAction::Create,
                "device".to_string(),
                format!("dev{}", i),
                AuditStatus::Success,
                "Created".to_string(),
            )).unwrap();
        }

        assert_eq!(logger.len(), 5);
    }

    #[test]
    fn test_audit_action_str_conversion() {
        assert_eq!(AuditAction::Create.as_str(), "create");
        assert_eq!(AuditAction::Delete.as_str(), "delete");
        assert_eq!(AuditAction::Login.as_str(), "login");
    }

    #[test]
    fn test_audit_status_str_conversion() {
        assert_eq!(AuditStatus::Success.as_str(), "success");
        assert_eq!(AuditStatus::Failure.as_str(), "failure");
        assert_eq!(AuditStatus::Partial.as_str(), "partial");
    }

    #[test]
    fn test_no_logs_for_nonexistent_actor() {
        let logger = AuditLogger::new(100);
        let logs = logger.query_by_actor("nonexistent").unwrap();
        assert_eq!(logs.len(), 0);
    }

    #[test]
    fn test_logger_clone() {
        let logger1 = AuditLogger::new(100);
        logger1.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        let logger2 = logger1.clone();
        assert_eq!(logger2.len(), 1);
        assert!(!logger2.is_empty());
    }

    #[test]
    fn test_multiple_resources() {
        let logger = AuditLogger::new(100);
        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "device".to_string(),
            "dev1".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        logger.log_action(AuditLog::new(
            "user1".to_string(),
            AuditAction::Create,
            "vlan".to_string(),
            "vlan10".to_string(),
            AuditStatus::Success,
            "Created".to_string(),
        )).unwrap();

        let devices = logger.query_by_resource("device", "dev1").unwrap();
        let vlans = logger.query_by_resource("vlan", "vlan10").unwrap();
        
        assert_eq!(devices.len(), 1);
        assert_eq!(vlans.len(), 1);
    }
}

pub use self::{AuditAction, AuditError, AuditLog, AuditLogger, AuditStatus};
