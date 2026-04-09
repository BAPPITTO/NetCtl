import React, { useState, useEffect } from 'react';
import { getState, getMetricsSummary } from '../api';
import DeviceList from './DeviceList';
import VLANManager from './VLANManager';
import QoSPanel from './QoSPanel';
import FlowVisualization from './FlowVisualization';
import PolicyBuilder from './PolicyBuilder';
import MetricsGraph from './MetricsGraph';
import AuditViewer from './AuditViewer';
import './Dashboard.css';

type Tab =
  | 'overview'
  | 'devices'
  | 'vlans'
  | 'qos'
  | 'flows'
  | 'policies'
  | 'metrics'
  | 'audit';

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState<Tab>('overview');
  const [state, setState] = useState<any>(null);
  const [metrics, setMetrics] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  // Fetch system state and metrics
  useEffect(() => {
    let isMounted = true;

    const fetchData = async () => {
      try {
        const [stateData, metricsData] = await Promise.all([
          getState(),
          getMetricsSummary(),
        ]);

        if (!isMounted) return;

        setState(stateData?.data || {});
        setMetrics(metricsData?.data || {});
      } catch (error) {
        console.error('Failed to fetch data:', error);
      } finally {
        if (isMounted) setLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 2000);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, []);

  if (loading) {
    return <div className="dashboard-loading">Initializing NetCtl...</div>;
  }

  const deviceCount = Object.keys(state?.devices || {}).length;
  const vlanCount = Object.keys(state?.vlans || {}).length;

  return (
    <div className="dashboard">
      <header className="dashboard-header">
        <h1>NetCtl - Network Control Engine</h1>
        <div className="status-bar">
          <span className="status-item">System Active</span>
          <span className="status-item">
            IPv4 Forwarding: {state?.ipv4_forwarding_enabled ? 'Enabled' : 'Disabled'}
          </span>
          <span className="status-item">Devices: {deviceCount}</span>
          <span className="status-item">VLANs: {vlanCount}</span>
        </div>
      </header>

      <nav className="dashboard-nav">
        {['overview', 'devices', 'vlans', 'qos', 'flows', 'policies', 'metrics', 'audit'].map((tab) => (
          <button
            key={tab}
            className={`nav-tab ${activeTab === tab ? 'active' : ''}`}
            onClick={() => setActiveTab(tab as Tab)}
          >
            {tab.charAt(0).toUpperCase() + tab.slice(1)}
          </button>
        ))}
      </nav>

      <main className="dashboard-content">
        {activeTab === 'overview' && (
          <section className="content-panel">
            <h2>System Overview</h2>
            <div className="metrics-grid">
              <div className="metric-card">
                <div className="metric-value">{metrics?.device_count || 0}</div>
                <div className="metric-label">Connected Devices</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{metrics?.total_rate_mbps || 0}</div>
                <div className="metric-label">Total Rate (Mbps)</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{metrics?.total_packets_dropped || 0}</div>
                <div className="metric-label">Packets Dropped</div>
              </div>
              <div className="metric-card">
                <div className="metric-value">{vlanCount}</div>
                <div className="metric-label">Active VLANs</div>
              </div>
            </div>
          </section>
        )}

        {activeTab === 'devices' && <DeviceList devices={state?.devices || {}} />}
        {activeTab === 'vlans' && <VLANManager vlans={state?.vlans || {}} />}
        {activeTab === 'qos' && <QoSPanel />}
        {activeTab === 'flows' && <FlowVisualization />}
        {activeTab === 'policies' && <PolicyBuilder />}
        {activeTab === 'metrics' && (
          <section className="content-panel">
            <h2>Network Metrics</h2>
            <MetricsGraph metricName="cpu_usage" />
            <MetricsGraph metricName="packet_rate" />
            <MetricsGraph metricName="bandwidth_usage" />
          </section>
        )}
        {activeTab === 'audit' && <AuditViewer />}
      </main>
    </div>
  );
}