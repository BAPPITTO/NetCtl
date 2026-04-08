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

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState<'overview' | 'devices' | 'vlans' | 'qos' | 'flows' | 'policies' | 'metrics' | 'audit'>('overview');
  const [state, setState] = useState<any>(null);
  const [metrics, setMetrics] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [stateData, metricsData] = await Promise.all([
          getState(),
          getMetricsSummary(),
        ]);
        setState(stateData.data);
        setMetrics(metricsData.data);
      } catch (error) {
        console.error('Failed to fetch data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 2000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return <div className="dashboard-loading">Initializing NetCtl...</div>;
  }

  return (
    <div className="dashboard">
      <header className="dashboard-header">
        <h1>🌐 NetCtl - Network Control Engine</h1>
        <div className="status-bar">
          <span className="status-item">● System Active</span>
          <span className="status-item">● IPv4 Forwarding: {state?.ipv4_forwarding_enabled ? 'Enabled' : 'Disabled'}</span>
          <span className="status-item">● Devices: {Object.keys(state?.devices || {}).length}</span>
          <span className="status-item">● VLANs: {Object.keys(state?.vlans || {}).length}</span>
        </div>
      </header>

      <nav className="dashboard-nav">
        <button 
          className={`nav-tab ${activeTab === 'overview' ? 'active' : ''}`}
          onClick={() => setActiveTab('overview')}
        >
          Overview
        </button>
        <button 
          className={`nav-tab ${activeTab === 'devices' ? 'active' : ''}`}
          onClick={() => setActiveTab('devices')}
        >
          Devices
        </button>
        <button 
          className={`nav-tab ${activeTab === 'vlans' ? 'active' : ''}`}
          onClick={() => setActiveTab('vlans')}
        >
          VLANs
        </button>
        <button 
          className={`nav-tab ${activeTab === 'qos' ? 'active' : ''}`}
          onClick={() => setActiveTab('qos')}
        >
          QoS
        </button>
        <button 
          className={`nav-tab ${activeTab === 'flows' ? 'active' : ''}`}
          onClick={() => setActiveTab('flows')}
        >
          🔀 Flows
        </button>
        <button 
          className={`nav-tab ${activeTab === 'policies' ? 'active' : ''}`}
          onClick={() => setActiveTab('policies')}
        >
          🔐 Policies
        </button>
        <button 
          className={`nav-tab ${activeTab === 'metrics' ? 'active' : ''}`}
          onClick={() => setActiveTab('metrics')}
        >
          📈 Metrics
        </button>
        <button 
          className={`nav-tab ${activeTab === 'audit' ? 'active' : ''}`}
          onClick={() => setActiveTab('audit')}
        >
          📋 Audit
        </button>
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
                <div className="metric-value">{Object.keys(state?.vlans || {}).length}</div>
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
