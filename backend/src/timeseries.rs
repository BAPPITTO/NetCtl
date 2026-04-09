//! Time-series metrics system with alert management

use crate::error::{Error, Result};
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single time-series data point
#[derive(Debug, Clone)]
pub struct TimeseriesPoint {
    pub timestamp: u64,
    pub value: f64,
    pub metric_name: String,
    pub tags: HashMap<String, String>,
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

    pub fn should_trigger(&self, value: f64) -> bool {
        match self.operator {
            AlertOperator::GreaterThan => value > self.threshold,
            AlertOperator::LessThan => value < self.threshold,
            AlertOperator::GreaterThanOrEqual => value >= self.threshold,
            AlertOperator::LessThanOrEqual => value <= self.threshold,
            AlertOperator::Equal => (value - self.threshold).abs() < f64::EPSILON,
        }
    }

    pub fn trigger(&mut self) {
        if self.state != AlertState::Active {
            self.state = AlertState::Active;
            self.triggered_at = Some(current_timestamp());
            self.acknowledged_at = None;
            self.resolved_at = None;
        }
    }

    pub fn acknowledge(&mut self) {
        if self.state == AlertState::Active {
            self.state = AlertState::Acknowledged;
            self.acknowledged_at = Some(current_timestamp());
        }
    }

    pub fn resolve(&mut self) {
        self.state = AlertState::Resolved;
        self.resolved_at = Some(current_timestamp());
    }
}

/// Thread-safe time-series database with bounded history
#[derive(Clone)]
pub struct TimeseriesDB {
    metrics: Arc<RwLock<BTreeMap<String, Vec<TimeseriesPoint>>>>,
    alerts: Arc<RwLock<Vec<MetricAlert>>>,
    max_points_per_metric: usize,
}

impl TimeseriesDB {
    pub fn new(max_points_per_metric: usize) -> Self {
        TimeseriesDB {
            metrics: Arc::new(RwLock::new(BTreeMap::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            max_points_per_metric,
        }
    }

    pub fn record_metric(&self, point: TimeseriesPoint) -> Result<()> {
        if point.value.is_nan() || point.value.is_infinite() {
            return Err(Error::StateError("Invalid metric value".into()));
        }

        let mut metrics = self
            .metrics
            .write()
            .map_err(|_| Error::StateError("Failed to acquire write lock".into()))?;

        let points = metrics.entry(point.metric_name.clone()).or_insert_with(Vec::new);
        points.push(point);

        if points.len() > self.max_points_per_metric {
            points.drain(0..(points.len() - self.max_points_per_metric));
        }

        Ok(())
    }

    pub fn query_range(&self, metric_name: &str, start_time: u64, end_time: u64) -> Result<Vec<TimeseriesPoint>> {
        let metrics = self
            .metrics
            .read()
            .map_err(|_| Error::StateError("Failed to acquire read lock".into()))?;

        let points = metrics.get(metric_name)
            .map(|v| v.iter()
                 .filter(|p| p.timestamp >= start_time && p.timestamp <= end_time)
                 .cloned()
                 .collect())
            .unwrap_or_default();

        Ok(points)
    }

    pub fn get_latest(&self, metric_name: &str) -> Result<Option<TimeseriesPoint>> {
        let metrics = self
            .metrics
            .read()
            .map_err(|_| Error::StateError("Failed to acquire read lock".into()))?;

        Ok(metrics.get(metric_name).and_then(|points| points.last().cloned()))
    }

    pub fn add_alert(&self, alert: MetricAlert) -> Result<()> {
        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| Error::StateError("Failed to acquire write lock".into()))?;
        alerts.push(alert);
        Ok(())
    }

    pub fn get_alerts(&self) -> Result<Vec<MetricAlert>> {
        let alerts = self
            .alerts
            .read()
            .map_err(|_| Error::StateError("Failed to acquire read lock".into()))?;
        Ok(alerts.clone())
    }

    pub fn check_alerts(&self) -> Result<Vec<String>> {
        let metrics = self
            .metrics
            .read()
            .map_err(|_| Error::StateError("Failed to acquire read lock".into()))?;

        let mut triggered_alerts = Vec::new();
        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| Error::StateError("Failed to acquire write lock".into()))?;

        for alert in alerts.iter_mut() {
            if let Some(points) = metrics.get(&alert.metric_name) {
                if let Some(latest) = points.last() {
                    if alert.should_trigger(latest.value) && alert.state != AlertState::Resolved {
                        alert.trigger();
                        triggered_alerts.push(alert.id.clone());
                    } else if !alert.should_trigger(latest.value) && alert.state != AlertState::Resolved {
                        alert.resolve();
                    }
                }
            }
        }

        Ok(triggered_alerts)
    }

    pub fn acknowledge_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self
            .alerts
            .write()
            .map_err(|_| Error::StateError("Failed to acquire write lock".into()))?;

        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledge();
            Ok(())
        } else {
            Err(Error::StateError(format!("Alert not found: {}", alert_id)))
        }
    }

    pub fn get_stats(&self, metric_name: &str, start_time: u64, end_time: u64) -> Result<MetricStats> {
        let points = self.query_range(metric_name, start_time, end_time)?;

        if points.is_empty() {
            return Err(Error::StateError("No data points found".into()));
        }

        let values: Vec<f64> = points.iter().map(|p| p.value).collect();
        let min = *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let avg = values.iter().sum::<f64>() / values.len() as f64;

        Ok(MetricStats {
            min,
            max,
            avg,
            count: values.len(),
        })
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

pub use self::{AlertOperator, AlertState, MetricAlert, MetricStats, TimeseriesDB, TimeseriesPoint};