use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::collections::HashMap;

/// Device metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetrics {
    pub mac: String,
    pub packets_sent: u64,
    pub packets_dropped: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub current_rate_mbps: f64,
    pub timestamp: String,
}

impl DeviceMetrics {
    /// Create a new metrics record for a device
    pub fn new(mac: String) -> Self {
        Self {
            mac,
            packets_sent: 0,
            packets_dropped: 0,
            bytes_sent: 0,
            bytes_received: 0,
            current_rate_mbps: 0.0,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Live metrics collector
#[derive(Debug, Default)]
pub struct MetricsCollector {
    device_metrics: HashMap<String, DeviceMetrics>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update metrics for a specific device
    pub fn update_device_metrics(&mut self, metrics: DeviceMetrics) {
        self.device_metrics.insert(metrics.mac.clone(), metrics);
    }

    /// Get metrics for all devices
    pub fn get_all_metrics(&self) -> Vec<DeviceMetrics> {
        self.device_metrics.values().cloned().collect()
    }

    /// Get metrics for a specific device
    pub fn get_device_metrics(&self, mac: &str) -> Option<DeviceMetrics> {
        self.device_metrics.get(mac).cloned()
    }

    /// Generate aggregated metrics summary
    pub fn get_summary(&self) -> MetricsSummary {
        let mut summary = MetricsSummary::default();
        summary.device_count = self.device_metrics.len();
        summary.timestamp = Utc::now().to_rfc3339();

        for metrics in self.device_metrics.values() {
            summary.total_packets_sent += metrics.packets_sent;
            summary.total_packets_dropped += metrics.packets_dropped;
            summary.total_bytes_sent += metrics.bytes_sent;
            summary.total_bytes_received += metrics.bytes_received;
            summary.total_rate_mbps += metrics.current_rate_mbps;
        }

        summary
    }
}

/// Aggregated metrics summary
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MetricsSummary {
    pub total_packets_sent: u64,
    pub total_packets_dropped: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub total_rate_mbps: f64,
    pub device_count: usize,
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        let metrics = DeviceMetrics::new("aa:bb:cc:dd:ee:ff".to_string());
        collector.update_device_metrics(metrics);

        assert_eq!(collector.get_all_metrics().len(), 1);
    }

    #[test]
    fn test_metrics_summary() {
        let mut collector = MetricsCollector::new();
        let mut metrics = DeviceMetrics::new("aa:bb:cc:dd:ee:ff".to_string());
        metrics.bytes_sent = 1000;
        collector.update_device_metrics(metrics);

        let summary = collector.get_summary();
        assert_eq!(summary.device_count, 1);
        assert_eq!(summary.total_bytes_sent, 1000);
    }
}