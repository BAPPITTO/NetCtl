import React, { useEffect, useState } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface Flow {
  src_ip: string;
  dst_ip: string;
  src_port: number;
  dst_port: number;
  protocol: string;
  packets: number;
  bytes: number;
}

interface FlowVisualizationProps {
  apiUrl?: string;
}

const FlowVisualization: React.FC<FlowVisualizationProps> = ({ apiUrl = 'http://localhost:3001' }) => {
  const [flows, setFlows] = useState<Flow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedFlow, setSelectedFlow] = useState<Flow | null>(null);

  useEffect(() => {
    const fetchFlows = async () => {
      try {
        setLoading(true);
        const response = await fetch(`${apiUrl}/api/flows?limit=50`);
        const data = await response.json();
        if (data.success && Array.isArray(data.data?.flows)) {
          setFlows(data.data.flows);
          setError(null);
        } else {
          setError(data.error || 'Failed to fetch flows');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setLoading(false);
      }
    };

    fetchFlows();
    const interval = setInterval(fetchFlows, 5000);
    return () => clearInterval(interval);
  }, [apiUrl]);

  if (loading) return <div className="flow-visualization">Loading flows...</div>;
  if (error) return <div className="flow-visualization error">{error}</div>;

  const topFlows = [...flows].sort((a, b) => b.bytes - a.bytes).slice(0, 10);
  const chartData = topFlows.map(flow => ({
    name: `${flow.src_ip.split('.').slice(-1)[0]} → ${flow.dst_ip.split('.').slice(-1)[0]}:${flow.dst_port}`,
    bytes: flow.bytes,
    packets: flow.packets,
  }));

  return (
    <div className="flow-visualization" style={{ fontFamily: 'Times New Roman, serif' }}>
      <div className="flow-header" style={{ color: '#00ff00', fontWeight: 'bold', marginBottom: 15 }}>
        Active Network Flows ({flows.length})
      </div>

      <div className="flow-stats" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: 15, marginBottom: 20 }}>
        <div className="flow-stat" style={{ padding: 10, background: 'rgba(0,255,0,0.1)', border: '1px solid rgba(0,255,0,0.3)', borderRadius: 4, color: '#00ff00' }}>
          <div className="flow-stat-label" style={{ fontSize: '0.9em', opacity: 0.7 }}>Total Flows</div>
          <div className="flow-stat-value" style={{ fontSize: '1.2em', fontWeight: 'bold' }}>{flows.length}</div>
        </div>
        <div className="flow-stat" style={{ padding: 10, background: 'rgba(0,255,0,0.1)', border: '1px solid rgba(0,255,0,0.3)', borderRadius: 4, color: '#00ff00' }}>
          <div className="flow-stat-label" style={{ fontSize: '0.9em', opacity: 0.7 }}>Total Bytes</div>
          <div className="flow-stat-value" style={{ fontSize: '1.2em', fontWeight: 'bold' }}>{(flows.reduce((sum, f) => sum + f.bytes, 0) / 1024 / 1024).toFixed(2)} MB</div>
        </div>
        <div className="flow-stat" style={{ padding: 10, background: 'rgba(0,255,0,0.1)', border: '1px solid rgba(0,255,0,0.3)', borderRadius: 4, color: '#00ff00' }}>
          <div className="flow-stat-label" style={{ fontSize: '0.9em', opacity: 0.7 }}>Total Packets</div>
          <div className="flow-stat-value" style={{ fontSize: '1.2em', fontWeight: 'bold' }}>{flows.reduce((sum, f) => sum + f.packets, 0)}</div>
        </div>
      </div>

      {chartData.length > 0 && (
        <div className="chart-container" style={{ margin: '20px 0', background: 'rgba(0,255,0,0.05)', padding: 10, borderRadius: 4 }}>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={chartData}>
              <CartesianGrid stroke="rgba(0,255,0,0.1)" />
              <XAxis dataKey="name" stroke="#00ff00" tick={{ fontSize: 12 }} angle={-45} textAnchor="end" height={80} />
              <YAxis stroke="#00ff00" />
              <Tooltip contentStyle={{ background: 'rgba(0,0,0,0.8)', border: '1px solid #00ff00' }} />
              <Legend />
              <Line type="monotone" dataKey="bytes" stroke="#00ff00" isAnimationActive={false} />
              <Line type="monotone" dataKey="packets" stroke="#ff00ff" isAnimationActive={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      )}

      <div className="flow-list">
        <div style={{ marginBottom: 10, color: '#00ff00', fontWeight: 'bold' }}>Top Flows by Bytes</div>
        {topFlows.map((flow, idx) => (
          <div
            key={idx}
            className={`flow-item ${selectedFlow === flow ? 'active' : ''}`}
            onClick={() => setSelectedFlow(flow)}
            style={{ padding: 10, background: 'rgba(0,255,0,0.08)', borderLeft: '3px solid rgba(0,255,0,0.3)', margin: '5px 0', cursor: 'pointer', fontFamily: 'Times New Roman, serif', fontSize: '0.9em', color: '#00ff00' }}
          >
            {flow.src_ip} → {flow.dst_ip}:{flow.dst_port} ({flow.protocol.toUpperCase()})
            <span style={{ opacity: 0.6 }}> {(flow.bytes / 1024).toFixed(2)} KB</span>
          </div>
        ))}
      </div>

      {selectedFlow && (
        <div className="flow-details" style={{ marginTop: 15, padding: 15, background: 'rgba(0,255,0,0.05)', border: '1px solid rgba(0,255,0,0.2)', borderRadius: 4, fontFamily: 'Times New Roman, serif', fontSize: '0.85em', lineHeight: 1.6, color: '#00ff00' }}>
          <div><strong>Source IP:</strong> {selectedFlow.src_ip}</div>
          <div><strong>Source Port:</strong> {selectedFlow.src_port}</div>
          <div><strong>Destination IP:</strong> {selectedFlow.dst_ip}</div>
          <div><strong>Destination Port:</strong> {selectedFlow.dst_port}</div>
          <div><strong>Protocol:</strong> {selectedFlow.protocol}</div>
          <div><strong>Packets:</strong> {selectedFlow.packets}</div>
          <div><strong>Bytes:</strong> {(selectedFlow.bytes / 1024 / 1024).toFixed(4)} MB</div>
        </div>
      )}
    </div>
  );
};

export default FlowVisualization;