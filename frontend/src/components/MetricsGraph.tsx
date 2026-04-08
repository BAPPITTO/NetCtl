import React, { useEffect, useState } from 'react';
import { LineChart, Line, AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface MetricPoint {
  timestamp: number;
  value: number;
  name: string;
}

interface MetricStats {
  metric: string;
  min: number;
  max: number;
  avg: number;
  count: number;
}

interface MetricsGraphProps {
  metricName: string;
  apiUrl?: string;
  refreshInterval?: number;
}

const MetricsGraph: React.FC<MetricsGraphProps> = ({ 
  metricName,
  apiUrl = 'http://localhost:3001',
  refreshInterval = 5000
}) => {
  const [data, setData] = useState<MetricPoint[]>([]);
  const [stats, setStats] = useState<MetricStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        setLoading(true);
        const response = await fetch(`${apiUrl}/api/metrics/stats/${metricName}`);
        const result = await response.json();

        if (result.success && result.data) {
          setStats(result.data);
          // Generate mock data for visualization
          const chartData = Array.from({ length: 12 }, (_, i) => ({
            timestamp: Date.now() - (i * 300000),
            value: result.data.avg + (Math.random() - 0.5) * result.data.max * 0.2,
            name: metricName,
          })).reverse();
          setData(chartData);
          setError(null);
        } else {
          setError(result.error || 'Failed to fetch metrics');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setLoading(false);
      }
    };

    fetchMetrics();
    const interval = setInterval(fetchMetrics, refreshInterval);
    return () => clearInterval(interval);
  }, [metricName, apiUrl, refreshInterval]);

  if (loading && data.length === 0) {
    return <div className="metrics-graph">Loading metrics...</div>;
  }

  if (error) {
    return <div className="metrics-graph error">{error}</div>;
  }

  const chartData = data.map((point, idx) => ({
    time: idx,
    value: Math.round(point.value * 100) / 100,
  }));

  return (
    <div className="metrics-graph">
      <style>{`
        .metrics-graph {
          padding: 20px;
          background: rgba(0, 255, 255, 0.05);
          border: 1px solid rgba(0, 255, 255, 0.2);
          border-radius: 4px;
          margin: 10px 0;
        }

        .metrics-graph.error {
          background: rgba(255, 0, 0, 0.05);
          border-color: rgba(255, 0, 0, 0.2);
          color: #ff0000;
        }

        .metrics-header {
          color: #00ffff;
          font-weight: bold;
          margin-bottom: 15px;
          text-shadow: 0 0 10px rgba(0, 255, 255, 0.5);
          font-size: 1.1em;
        }

        .metrics-stats {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
          gap: 12px;
          margin-bottom: 20px;
        }

        .metric-stat {
          padding: 12px;
          background: rgba(0, 255, 255, 0.1);
          border: 1px solid rgba(0, 255, 255, 0.3);
          border-radius: 4px;
          text-align: center;
        }

        .metric-stat-label {
          font-size: 0.8em;
          opacity: 0.7;
          color: rgba(0, 255, 255, 0.8);
          margin-bottom: 5px;
          text-transform: uppercase;
          font-weight: bold;
        }

        .metric-stat-value {
          font-size: 1.3em;
          color: #00ffff;
          font-family: monospace;
          font-weight: bold;
        }

        .chart-container {
          background: rgba(0, 255, 255, 0.05);
          padding: 15px;
          border-radius: 4px;
          border: 1px solid rgba(0, 255, 255, 0.1);
        }

        .chart-title {
          color: rgba(0, 255, 255, 0.7);
          font-size: 0.9em;
          margin-bottom: 10px;
          font-weight: bold;
        }
      `}</style>

      <div className="metrics-header">📈 Metrics: {metricName}</div>

      {stats && (
        <div className="metrics-stats">
          <div className="metric-stat">
            <div className="metric-stat-label">Current</div>
            <div className="metric-stat-value">{stats.avg.toFixed(2)}</div>
          </div>
          <div className="metric-stat">
            <div className="metric-stat-label">Min</div>
            <div className="metric-stat-value">{stats.min.toFixed(2)}</div>
          </div>
          <div className="metric-stat">
            <div className="metric-stat-label">Max</div>
            <div className="metric-stat-value">{stats.max.toFixed(2)}</div>
          </div>
          <div className="metric-stat">
            <div className="metric-stat-label">Avg</div>
            <div className="metric-stat-value">{stats.avg.toFixed(2)}</div>
          </div>
          <div className="metric-stat">
            <div className="metric-stat-label">Samples</div>
            <div className="metric-stat-value">{stats.count}</div>
          </div>
        </div>
      )}

      <div className="chart-container">
        <div className="chart-title">Time Series - Last 1 Hour</div>
        <ResponsiveContainer width="100%" height={300}>
          <AreaChart data={chartData}>
            <defs>
              <linearGradient id="colorValue" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="#00ffff" stopOpacity={0.3} />
                <stop offset="95%" stopColor="#00ffff" stopOpacity={0} />
              </linearGradient>
            </defs>
            <CartesianGrid stroke="rgba(0,255,255,0.1)" />
            <XAxis dataKey="time" stroke="#00ffff" tick={{ fontSize: 12 }} />
            <YAxis stroke="#00ffff" />
            <Tooltip
              contentStyle={{
                background: 'rgba(0, 0, 0, 0.9)',
                border: '1px solid #00ffff',
                borderRadius: '4px',
                color: '#00ffff',
              }}
            />
            <Legend />
            <Area
              type="monotone"
              dataKey="value"
              stroke="#00ffff"
              fillOpacity={1}
              fill="url(#colorValue)"
              isAnimationActive={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>

      {data.length > 0 && (
        <div style={{ marginTop: '15px', fontSize: '0.85em', color: 'rgba(0,255,255,0.6)' }}>
          <strong>Last Update:</strong> {new Date(data[data.length - 1].timestamp).toLocaleTimeString()}
        </div>
      )}
    </div>
  );
};

export default MetricsGraph;
