//! Time-series metrics system with alert management
//! 
//! This module provides time-series data storage with:
//! - Bounded history management
//! - Range-based queries
//! - Metric alerting with threshold-based triggering
//! - Statistical analysis (min, max, avg)

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single time-series data point
#[derive(Debug, Clone)]
pub struct TimeseriesPoint {
    pub timestamp: u64,
    pub value: f64,
    pub metric_name: String,
    pub tags: std::collections::HashMap<String, String>,
}

/// Operators for alert thresholds
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
}

/// Alert states in their lifecycle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertState {
    Active,
    Acknowledged,
    Resolved,
}

/// Represents a metric alert with threshold and triggering conditions
#[derive(Debug, Clone)]
pub struct MetricAlert {
    pub id: String,
    pub metric_name: String,
    pub threshold: f64,
    pub operator: AlertOperator,
    pub state: AlertState,
    pub triggered_at: Option<u64>,
    pub acknowledged_at: Option<u64>,
    pub resolved_at: Option<u64>,
}

impl MetricAlert {
    /// Create a new metric alert
    pub fn new(id: String, metric_name: String, threshold: f64, operator: AlertOperator) -> Self {
        MetricAlert {
            id,
            metric_name,
            threshold,
            operator,
            state: AlertState::Active,
            triggered_at: None,
            acknowledged_at: None,
            resolved_at: None,
        }
    }

    /// Check if alert should trigger based on value
    pub fn should_trigger(&self, value: f64) -> bool {
        match self.operator {
            AlertOperator::GreaterThan => value > self.threshold,
            AlertOperator::LessThan => value < self.threshold,
            AlertOperator::GreaterThanOrEqual => value >= self.threshold,
            AlertOperator::LessThanOrEqual => value <= self.threshold,
            AlertOperator::Equal => (value - self.threshold).abs() < f64::EPSILON,
        }
    }

    /// Trigger the alert (transition to active)
    pub fn trigger(&mut self) {
        if self.state != AlertState::Active {
            self.state = AlertState::Active;
            self.triggered_at = Some(current_timestamp());
            self.acknowledged_at = None;
            self.resolved_at = None;
        }
    }

    /// Acknowledge the alert
    pub fn acknowledge(&mut self) {
        if self.state == AlertState::Active {
            self.state = AlertState::Acknowledged;
            self.acknowledged_at = Some(current_timestamp());
        }
    }

    /// Resolve the alert
    pub fn resolve(&mut self) {
        self.state = AlertState::Resolved;
        self.resolved_at = Some(current_timestamp());
    }
}

/// Thread-safe time-series database with bounded history
pub struct TimeseriesDB {
    metrics: Arc<RwLock<BTreeMap<String, Vec<TimeseriesPoint>>>>,
    alerts: Arc<RwLock<Vec<MetricAlert>>>,
    max_points_per_metric: usize,
}

impl TimeseriesDB {
    /// Create a new time-series database with bounded history
    pub fn new(max_points_per_metric: usize) -> Self {
        TimeseriesDB {
            metrics: Arc::new(RwLock::new(BTreeMap::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            max_points_per_metric,
        }
    }

    /// Record a new metric point
    pub fn record_metric(&self, point: TimeseriesPoint) -> Result<(), String> {
        if point.value.is_nan() || point.value.is_infinite() {
            return Err("Invalid metric value".to_string());
        }

        let mut metrics = self
            .metrics
            .write()
            .map_err(|_| "Failed to acquire write lock".to_string())?;

        let points = metrics
            .entry(point.metric_name.clone())
            .or_insert_with(Vec::new);

        points.push(point);

        // Keep only the most recent max_points_per_metric entries
        if points.len() > self.max_points_per_metric {
            let excess = points.len() - self.max_points_per_metric;
            points.drain(0..excess);
        }

        Ok(())
    }

    /// Query metrics within a time range (inclusive)
    pub fn query_range(
        &self,
        metric_name: &str,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<TimeseriesPoint>, String> {
        let metrics = self
            .metrics
            .read()
            .map_err(|_| "Failed to acquire read lock".to_string())?;

        if let Some(points) = metrics.get(metric_name) {
            let filtered: Vec<TimeseriesPoint> = points
                .iter()
                .filter(|p| p.timestamp >= start_time && p.timestamp <= end_time)
                .cloned()
                .collect();
            Ok(filtered)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get the latest metric point
    pub fn get_latest(&self, metric_name: &str) -> Result<Option<TimeseriesPoint>, String> {
        let metrics = self
            .metrics
            .read()
            .map_err(|_| "Failed to acquire read lock".to_string())?;

        Ok(metrics.get(metric_name).and_then(|points| points.last().cloned()))
    }

    /// Add an alert to the system
    pub fn add_alert(&self, alert: MetricAlert) -> Result<(), String> {
        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| "Failed to acquire write lock".to_string())?;
        alerts.push(alert);
        Ok(())
    }

    /// Get all alerts
    pub fn get_alerts(&self) -> Result<Vec<MetricAlert>, String> {
        let alerts = self
            .alerts
            .read()
            .map_err(|_| "Failed to acquire read lock".to_string())?;
        Ok(alerts.clone())
    }

    /// Check and update alerts based on current metrics
    pub fn check_alerts(&self) -> Result<Vec<String>, String> {
        let mut triggered_alerts = Vec::new();
        let metrics = self
            .metrics
            .read()
            .map_err(|_| "Failed to acquire read lock".to_string())?;

        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| "Failed to acquire write lock".to_string())?;

        for alert in alerts.iter_mut() {
            if let Some(points) = metrics.get(&alert.metric_name) {
                if let Some(latest) = points.last() {
                    if alert.should_trigger(latest.value) && alert.state == AlertState::Active {
                        alert.trigger();
                        triggered_alerts.push(alert.id.clone());
                    } else if !alert.should_trigger(latest.value)
                        && alert.state != AlertState::Resolved
                    {
                        alert.resolve();
                    }
                }
            }
        }

        Ok(triggered_alerts)
    }

    /// Acknowledge an alert by ID
    pub fn acknowledge_alert(&self, alert_id: &str) -> Result<(), String> {
        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| "Failed to acquire write lock".to_string())?;

        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledge();
            Ok(())
        } else {
            Err(format!("Alert not found: {}", alert_id))
        }
    }

    /// Get metric statistics
    pub fn get_stats(
        &self,
        metric_name: &str,
        start_time: u64,
        end_time: u64,
    ) -> Result<MetricStats, String> {
        let points = self.query_range(metric_name, start_time, end_time)?;

        if points.is_empty() {
            return Err("No data points found".to_string());
        }

        let values: Vec<f64> = points.iter().map(|p| p.value).collect();
        let min = values
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max = values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let avg = values.iter().sum::<f64>() / values.len() as f64;

        Ok(MetricStats {
            min,
            max,
            avg,
            count: values.len(),
        })
    }
}

impl Clone for TimeseriesDB {
    fn clone(&self) -> Self {
        TimeseriesDB {
            metrics: Arc::clone(&self.metrics),
            alerts: Arc::clone(&self.alerts),
            max_points_per_metric: self.max_points_per_metric,
        }
    }
}

/// Statistics for a metric over a time range
#[derive(Debug, Clone)]
pub struct MetricStats {
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: usize,
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeseries_point_creation() {
        let point = TimeseriesPoint {
            timestamp: 1000,
            value: 42.5,
            metric_name: "cpu_usage".to_string(),
            tags: std::collections::HashMap::new(),
        };
        assert_eq!(point.value, 42.5);
    }

    #[test]
    fn test_metric_alert_creation() {
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu_usage".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        assert_eq!(alert.state, AlertState::Active);
        assert_eq!(alert.threshold, 80.0);
    }

    #[test]
    fn test_alert_should_trigger_greater_than() {
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        assert!(alert.should_trigger(85.0));
        assert!(!alert.should_trigger(75.0));
        assert!(!alert.should_trigger(80.0));
    }

    #[test]
    fn test_alert_should_trigger_less_than() {
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "memory".to_string(),
            10.0,
            AlertOperator::LessThan,
        );
        assert!(alert.should_trigger(5.0));
        assert!(!alert.should_trigger(15.0));
    }

    #[test]
    fn test_alert_lifecycle() {
        let mut alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        assert_eq!(alert.state, AlertState::Active);

        alert.trigger();
        assert_eq!(alert.state, AlertState::Active);
        assert!(alert.triggered_at.is_some());

        alert.acknowledge();
        assert_eq!(alert.state, AlertState::Acknowledged);
        assert!(alert.acknowledged_at.is_some());

        alert.resolve();
        assert_eq!(alert.state, AlertState::Resolved);
        assert!(alert.resolved_at.is_some());
    }

    #[test]
    fn test_timeseries_db_creation() {
        let db = TimeseriesDB::new(100);
        let alerts = db.get_alerts();
        assert!(alerts.is_ok());
        assert_eq!(alerts.unwrap().len(), 0);
    }

    #[test]
    fn test_record_metric() {
        let db = TimeseriesDB::new(100);
        let point = TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        };
        let result = db.record_metric(point);
        assert!(result.is_ok());
    }

    #[test]
    fn test_record_invalid_metric() {
        let db = TimeseriesDB::new(100);
        let point = TimeseriesPoint {
            timestamp: 1000,
            value: f64::NAN,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        };
        let result = db.record_metric(point);
        assert!(result.is_err());
    }

    #[test]
    fn test_query_range() {
        let db = TimeseriesDB::new(100);
        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();
        db.record_metric(TimeseriesPoint {
            timestamp: 2000,
            value: 60.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let result = db.query_range("cpu", 1500, 2500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_get_latest() {
        let db = TimeseriesDB::new(100);
        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();
        db.record_metric(TimeseriesPoint {
            timestamp: 2000,
            value: 70.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let latest = db.get_latest("cpu");
        assert!(latest.is_ok());
        assert_eq!(latest.unwrap().unwrap().value, 70.0);
    }

    #[test]
    fn test_add_alert() {
        let db = TimeseriesDB::new(100);
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        let result = db.add_alert(alert);
        assert!(result.is_ok());

        let alerts = db.get_alerts().unwrap();
        assert_eq!(alerts.len(), 1);
    }

    #[test]
    fn test_check_alerts() {
        let db = TimeseriesDB::new(100);
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        db.add_alert(alert).unwrap();

        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 85.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let triggered = db.check_alerts();
        assert!(triggered.is_ok());
    }

    #[test]
    fn test_acknowledge_alert() {
        let db = TimeseriesDB::new(100);
        let alert = MetricAlert::new(
            "alert1".to_string(),
            "cpu".to_string(),
            80.0,
            AlertOperator::GreaterThan,
        );
        db.add_alert(alert).unwrap();

        let result = db.acknowledge_alert("alert1");
        assert!(result.is_ok());

        let alerts = db.get_alerts().unwrap();
        assert_eq!(alerts[0].state, AlertState::Acknowledged);
    }

    #[test]
    fn test_bounded_history() {
        let db = TimeseriesDB::new(5);
        for i in 0..10 {
            db.record_metric(TimeseriesPoint {
                timestamp: 1000 + i,
                value: 50.0 + i as f64,
                metric_name: "cpu".to_string(),
                tags: std::collections::HashMap::new(),
            })
            .unwrap();
        }

        let all_points = db.query_range("cpu", 0, 10000).unwrap();
        assert_eq!(all_points.len(), 5);
    }

    #[test]
    fn test_get_stats() {
        let db = TimeseriesDB::new(100);
        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();
        db.record_metric(TimeseriesPoint {
            timestamp: 2000,
            value: 60.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();
        db.record_metric(TimeseriesPoint {
            timestamp: 3000,
            value: 70.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let stats = db.get_stats("cpu", 0, 5000);
        assert!(stats.is_ok());
        let s = stats.unwrap();
        assert_eq!(s.min, 50.0);
        assert_eq!(s.max, 70.0);
        assert_eq!(s.avg, 60.0);
        assert_eq!(s.count, 3);
    }

    #[test]
    fn test_multiple_metrics() {
        let db = TimeseriesDB::new(100);
        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();
        db.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 80.0,
            metric_name: "memory".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let cpu_latest = db.get_latest("cpu").unwrap().unwrap();
        let mem_latest = db.get_latest("memory").unwrap().unwrap();
        assert_eq!(cpu_latest.value, 50.0);
        assert_eq!(mem_latest.value, 80.0);
    }

    #[test]
    fn test_db_clone() {
        let db1 = TimeseriesDB::new(100);
        db1.record_metric(TimeseriesPoint {
            timestamp: 1000,
            value: 50.0,
            metric_name: "cpu".to_string(),
            tags: std::collections::HashMap::new(),
        })
        .unwrap();

        let db2 = db1.clone();
        let latest = db2.get_latest("cpu");
        assert!(latest.is_ok());
        assert_eq!(latest.unwrap().unwrap().value, 50.0);
    }
}

pub use self::{AlertOperator, AlertState, MetricAlert, MetricStats, TimeseriesDB, TimeseriesPoint};
