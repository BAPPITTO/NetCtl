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
pub struct MetricsCollector {
    device_metrics: HashMap<String, DeviceMetrics>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            device_metrics: HashMap::new(),
        }
    }

    /// Update device metrics
    pub fn update_device_metrics(&mut self, metrics: DeviceMetrics) {
        self.device_metrics.insert(metrics.mac.clone(), metrics);
    }

    /// Get all device metrics
    pub fn get_all_metrics(&self) -> Vec<DeviceMetrics> {
        self.device_metrics.values().cloned().collect()
    }

    /// Get metrics for specific device
    pub fn get_device_metrics(&self, mac: &str) -> Option<DeviceMetrics> {
        self.device_metrics.get(mac).cloned()
    }

    /// Generate summary metrics
    pub fn get_summary(&self) -> MetricsSummary {
        let mut total_packets_sent = 0u64;
        let mut total_packets_dropped = 0u64;
        let mut total_bytes_sent = 0u64;
        let mut total_bytes_received = 0u64;
        let mut total_rate_mbps = 0.0f64;

        for metrics in self.device_metrics.values() {
            total_packets_sent += metrics.packets_sent;
            total_packets_dropped += metrics.packets_dropped;
            total_bytes_sent += metrics.bytes_sent;
            total_bytes_received += metrics.bytes_received;
            total_rate_mbps += metrics.current_rate_mbps;
        }

        MetricsSummary {
            total_packets_sent,
            total_packets_dropped,
            total_bytes_sent,
            total_bytes_received,
            total_rate_mbps,
            device_count: self.device_metrics.len(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Aggregated metrics summary
#[derive(Debug, Serialize, Deserialize)]
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
