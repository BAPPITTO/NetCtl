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

const FlowVisualization: React.FC<FlowVisualizationProps> = ({ 
  apiUrl = 'http://localhost:3001' 
}) => {
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
        
        if (data.success && data.data?.flows) {
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

  if (loading) {
    return <div className="flow-visualization">Loading flows...</div>;
  }

  if (error) {
    return <div className="flow-visualization error">{error}</div>;
  }

  // Sort flows by bytes to show top flows
  const topFlows = [...flows].sort((a, b) => b.bytes - a.bytes).slice(0, 10);

  const chartData = topFlows.map(flow => ({
    name: `${flow.src_ip.split('.').slice(-1)[0]} → ${flow.dst_ip.split('.').slice(-1)[0]}:${flow.dst_port}`,
    bytes: flow.bytes,
    packets: flow.packets
  }));

  return (
    <div className="flow-visualization">
      <style>{`
        .flow-visualization {
          padding: 20px;
          background: rgba(0, 255, 0, 0.05);
          border: 1px solid rgba(0, 255, 0, 0.2);
          border-radius: 4px;
          margin: 10px 0;
        }

        .flow-visualization.error {
          background: rgba(255, 0, 0, 0.05);
          border-color: rgba(255, 0, 0, 0.2);
          color: #ff0000;
        }

        .flow-header {
          color: #00ff00;
          font-weight: bold;
          margin-bottom: 15px;
          text-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
        }

        .flow-stats {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
          gap: 15px;
          margin-bottom: 20px;
        }

        .flow-stat {
          padding: 10px;
          background: rgba(0, 255, 0, 0.1);
          border: 1px solid rgba(0, 255, 0, 0.3);
          border-radius: 4px;
          color: #00ff00;
          font-family: monospace;
        }

        .flow-stat-label {
          font-size: 0.9em;
          opacity: 0.7;
          margin-bottom: 5px;
        }

        .flow-stat-value {
          font-size: 1.2em;
          font-weight: bold;
        }

        .flow-list {
          margin-top: 20px;
          max-height: 400px;
          overflow-y: auto;
        }

        .flow-item {
          padding: 10px;
          background: rgba(0, 255, 0, 0.08);
          border-left: 3px solid rgba(0, 255, 0, 0.3);
          margin: 5px 0;
          cursor: pointer;
          transition: background 0.2s;
          font-family: monospace;
          font-size: 0.9em;
          color: #00ff00;
        }

        .flow-item:hover {
          background: rgba(0, 255, 0, 0.15);
          border-left-color: rgba(0, 255, 0, 0.6);
        }

        .flow-item.active {
          background: rgba(0, 255, 0, 0.2);
          border-left-color: rgba(0, 255, 0, 0.8);
        }

        .flow-details {
          margin-top: 15px;
          padding: 15px;
          background: rgba(0, 255, 0, 0.05);
          border: 1px solid rgba(0, 255, 0, 0.2);
          border-radius: 4px;
          color: #00ff00;
          font-family: monospace;
          font-size: 0.85em;
          line-height: 1.6;
        }

        .chart-container {
          margin: 20px 0;
          background: rgba(0, 255, 0, 0.05);
          padding: 10px;
          border-radius: 4px;
        }
      `}</style>

      <div className="flow-header">🔀 Active Network Flows ({flows.length})</div>

      <div className="flow-stats">
        <div className="flow-stat">
          <div className="flow-stat-label">Total Flows</div>
          <div className="flow-stat-value">{flows.length}</div>
        </div>
        <div className="flow-stat">
          <div className="flow-stat-label">Total Bytes</div>
          <div className="flow-stat-value">{(flows.reduce((sum, f) => sum + f.bytes, 0) / 1024 / 1024).toFixed(2)} MB</div>
        </div>
        <div className="flow-stat">
          <div className="flow-stat-label">Total Packets</div>
          <div className="flow-stat-value">{flows.reduce((sum, f) => sum + f.packets, 0)}</div>
        </div>
      </div>

      {chartData.length > 0 && (
        <div className="chart-container">
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={chartData}>
              <CartesianGrid stroke="rgba(0,255,0,0.1)" />
              <XAxis dataKey="name" stroke="#00ff00" tick={{ fontSize: 12 }} angle={-45} textAnchor="end" height={80} />
              <YAxis stroke="#00ff00" />
              <Tooltip contentStyle={{ background: 'rgba(0, 0, 0, 0.8)', border: '1px solid #00ff00' }} />
              <Legend />
              <Line type="monotone" dataKey="bytes" stroke="#00ff00" isAnimationActive={false} />
              <Line type="monotone" dataKey="packets" stroke="#ff00ff" isAnimationActive={false} />
            </LineChart>
          </ResponsiveContainer>
        </div>
      )}

      <div className="flow-list">
        <div style={{ marginBottom: '10px', color: '#00ff00', fontWeight: 'bold' }}>Top Flows by Bytes</div>
        {topFlows.map((flow, idx) => (
          <div
            key={idx}
            className={`flow-item ${selectedFlow === flow ? 'active' : ''}`}
            onClick={() => setSelectedFlow(flow)}
          >
            {flow.src_ip} → {flow.dst_ip}:{flow.dst_port} ({flow.protocol.toUpperCase()}) 
            <span style={{ opacity: 0.6 }}> {(flow.bytes / 1024).toFixed(2)} KB</span>
          </div>
        ))}
      </div>

      {selectedFlow && (
        <div className="flow-details">
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
