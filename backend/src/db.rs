use rusqlite::{Connection, params, Row};
use crate::state::{SystemState, Device, Vlan, DhcpScope};
use crate::error::{Error, Result};
use chrono::Utc;
use serde_json;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        
        let db = Database { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS state (
                id INTEGER PRIMARY KEY,
                version INTEGER NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                operations TEXT NOT NULL,
                rollback_stack TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS audit_log (
                id INTEGER PRIMARY KEY,
                action TEXT NOT NULL,
                details TEXT,
                timestamp TEXT NOT NULL
            );
            "#
        ).map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Save system state to database
    pub fn save_state(&self, state: &SystemState) -> Result<()> {
        let json_data = serde_json::to_string(state)
            .map_err(|e| Error::DatabaseError(e.to_string()))?;

        self.conn.execute(
            "INSERT INTO state (version, data, created_at) VALUES (?1, ?2, ?3)",
            params![state.version, json_data, Utc::now().to_rfc3339()],
        ).map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Load latest system state from database
    pub fn load_state(&self) -> Result<Option<SystemState>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM state ORDER BY created_at DESC LIMIT 1"
        ).map_err(|e| Error::DatabaseError(e.to_string()))?;

        let result = stmt.query_row([], |row| {
            row.get::<_, String>(0)
        });

        match result {
            Ok(json_data) => {
                let state = serde_json::from_str(&json_data)
                    .map_err(|e| Error::DatabaseError(e.to_string()))?;
                Ok(Some(state))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::DatabaseError(e.to_string())),
        }
    }

    /// Log an audit entry
    pub fn log_audit(&self, action: &str, details: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO audit_log (action, details, timestamp) VALUES (?1, ?2, ?3)",
            params![action, details, Utc::now().to_rfc3339()],
        ).map_err(|e| Error::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Get audit trail
    pub fn get_audit_trail(&self, limit: usize) -> Result<Vec<(String, Option<String>, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT action, details, timestamp FROM audit_log ORDER BY timestamp DESC LIMIT ?1"
        ).map_err(|e| Error::DatabaseError(e.to_string()))?;

        let trail = stmt.query_map(params![limit as i32], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
            ))
        }).map_err(|e| Error::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for entry in trail {
            result.push(entry.map_err(|e| Error::DatabaseError(e.to_string()))?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_creation() {
        let temp = NamedTempFile::new().unwrap();
        let db = Database::open(temp.path()).unwrap();
        // If no panic, database was created successfully
    }

    #[test]
    fn test_save_and_load_state() {
        let temp = NamedTempFile::new().unwrap();
        let db = Database::open(temp.path()).unwrap();
        
        let mut state = SystemState::new();
        state.ipv4_forwarding_enabled = true;
        
        db.save_state(&state).unwrap();
        let loaded = db.load_state().unwrap().unwrap();
        
        assert_eq!(loaded.ipv4_forwarding_enabled, true);
    }
}
