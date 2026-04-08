import React, { useEffect, useState } from 'react';

interface AuditEntry {
  id: string;
  actor_id: string;
  action: string;
  resource_type: string;
  resource_id: string;
  status: string;
  timestamp: string;
  details: string;
}

interface AuditViewerProps {
  apiUrl?: string;
  refreshInterval?: number;
}

type FilterBy = 'all' | 'actor' | 'action' | 'status';

const AuditViewer: React.FC<AuditViewerProps> = ({ 
  apiUrl = 'http://localhost:3001',
  refreshInterval = 10000
}) => {
  const [logs, setLogs] = useState<AuditEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<FilterBy>('all');
  const [filterValue, setFilterValue] = useState('');
  const [expandedId, setExpandedId] = useState<string | null>(null);

  useEffect(() => {
    const fetchLogs = async () => {
      try {
        setLoading(true);
        let url = `${apiUrl}/api/audit/logs?limit=100`;

        if (filterType === 'actor' && filterValue) {
          url = `${apiUrl}/api/audit/logs/actor/${filterValue}`;
        } else if (filterType === 'action' && filterValue) {
          url = `${apiUrl}/api/audit/logs/action/${filterValue}`;
        }

        const response = await fetch(url);
        const data = await response.json();

        if (data.success && data.data?.logs) {
          setLogs(data.data.logs);
          setError(null);
        } else {
          setError(data.error || 'Failed to fetch audit logs');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setLoading(false);
      }
    };

    fetchLogs();
    const interval = setInterval(fetchLogs, refreshInterval);
    return () => clearInterval(interval);
  }, [apiUrl, filterType, filterValue, refreshInterval]);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'success':
        return '#00ff00';
      case 'failure':
        return '#ff0000';
      case 'partial':
        return '#ffff00';
      default:
        return '#00ffff';
    }
  };

  const getActionIcon = (action: string) => {
    switch (action) {
      case 'create':
        return '➕';
      case 'delete':
        return '❌';
      case 'update':
        return '✏️';
      case 'read':
        return '👁️';
      case 'login':
        return '🔓';
      case 'logout':
        return '🔐';
      case 'export':
        return '📤';
      case 'configure':
        return '⚙️';
      case 'execute':
        return '▶️';
      default:
        return '📋';
    }
  };

  const filteredLogs = logs.filter(log => {
    if (filterType === 'status' && filterValue) {
      return log.status === filterValue;
    }
    return true;
  });

  return (
    <div className="audit-viewer">
      <style>{`
        .audit-viewer {
          padding: 20px;
          background: rgba(255, 255, 0, 0.05);
          border: 1px solid rgba(255, 255, 0, 0.2);
          border-radius: 4px;
          margin: 10px 0;
        }

        .audit-header {
          color: #ffff00;
          font-weight: bold;
          margin-bottom: 15px;
          text-shadow: 0 0 10px rgba(255, 255, 0, 0.5);
          font-size: 1.1em;
        }

        .audit-controls {
          display: flex;
          gap: 12px;
          margin-bottom: 15px;
          flex-wrap: wrap;
        }

        .filter-group {
          display: flex;
          gap: 8px;
          align-items: center;
        }

        .filter-label {
          color: rgba(255, 255, 0, 0.7);
          font-size: 0.9em;
          font-weight: bold;
        }

        .filter-select,
        .filter-input {
          padding: 6px 10px;
          background: rgba(255, 255, 0, 0.1);
          border: 1px solid rgba(255, 255, 0, 0.3);
          color: #ffff00;
          border-radius: 4px;
          font-family: monospace;
          font-size: 0.9em;
        }

        .filter-select:focus,
        .filter-input:focus {
          outline: none;
          border-color: rgba(255, 255, 0, 0.6);
          box-shadow: 0 0 10px rgba(255, 255, 0, 0.2);
        }

        .audit-stats {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
          gap: 10px;
          margin-bottom: 15px;
        }

        .audit-stat {
          padding: 10px;
          background: rgba(255, 255, 0, 0.08);
          border: 1px solid rgba(255, 255, 0, 0.2);
          border-radius: 4px;
          text-align: center;
          color: #ffff00;
          font-size: 0.9em;
        }

        .audit-stat-value {
          font-size: 1.3em;
          font-weight: bold;
          font-family: monospace;
        }

        .audit-log-list {
          max-height: 600px;
          overflow-y: auto;
        }

        .audit-log-entry {
          background: rgba(255, 255, 0, 0.08);
          border-left: 3px solid rgba(255, 255, 0, 0.3);
          padding: 12px;
          margin: 8px 0;
          border-radius: 4px;
          cursor: pointer;
          transition: all 0.2s;
          color: #ffff00;
          font-family: monospace;
          font-size: 0.85em;
          line-height: 1.5;
        }

        .audit-log-entry:hover {
          background: rgba(255, 255, 0, 0.12);
          border-left-color: rgba(255, 255, 0, 0.6);
          box-shadow: 0 0 10px rgba(255, 255, 0, 0.1);
        }

        .audit-log-entry.expanded {
          background: rgba(255, 255, 0, 0.15);
          border-left-color: rgba(255, 255, 0, 0.8);
        }

        .log-entry-header {
          display: grid;
          grid-template-columns: auto 1fr auto auto auto auto;
          gap: 15px;
          align-items: center;
          margin-bottom: 8px;
        }

        .log-action-icon {
          font-size: 1.2em;
          min-width: 20px;
        }

        .log-actor {
          font-weight: bold;
          opacity: 0.9;
        }

        .log-resource {
          font-size: 0.85em;
          opacity: 0.8;
        }

        .log-status {
          font-weight: bold;
          padding: 2px 8px;
          border-radius: 2px;
          background: rgba(0, 0, 0, 0.3);
        }

        .log-timestamp {
          font-size: 0.8em;
          opacity: 0.7;
          white-space: nowrap;
        }

        .log-details {
          background: rgba(0, 0, 0, 0.2);
          padding: 12px;
          border-radius: 4px;
          margin-top: 10px;
          font-size: 0.8em;
          opacity: 0.8;
          line-height: 1.6;
          word-break: break-word;
        }

        .empty-state {
          color: rgba(255, 255, 0, 0.5);
          text-align: center;
          padding: 30px;
          font-style: italic;
        }

        .error {
          background: rgba(255, 0, 0, 0.1);
          border: 1px solid rgba(255, 0, 0, 0.3);
          color: #ff0000;
          padding: 10px;
          border-radius: 4px;
          margin-bottom: 15px;
        }
      `}</style>

      <div className="audit-header">📋 Audit Log Viewer</div>

      {error && <div className="error">⚠️ {error}</div>}

      <div className="audit-controls">
        <div className="filter-group">
          <span className="filter-label">Filter by:</span>
          <select
            className="filter-select"
            value={filterType}
            onChange={(e) => {
              setFilterType(e.target.value as FilterBy);
              setFilterValue('');
            }}
          >
            <option value="all">All</option>
            <option value="actor">Actor (User)</option>
            <option value="action">Action</option>
            <option value="status">Status</option>
          </select>
        </div>

        {filterType !== 'all' && (
          <div className="filter-group">
            {filterType === 'status' ? (
              <select
                className="filter-select"
                value={filterValue}
                onChange={(e) => setFilterValue(e.target.value)}
              >
                <option value="">All</option>
                <option value="success">Success</option>
                <option value="failure">Failure</option>
                <option value="partial">Partial</option>
              </select>
            ) : (
              <input
                className="filter-input"
                type="text"
                placeholder={`Search by ${filterType}...`}
                value={filterValue}
                onChange={(e) => setFilterValue(e.target.value)}
              />
            )}
          </div>
        )}
      </div>

      <div className="audit-stats">
        <div className="audit-stat">
          <div className="filter-label">Total Entries</div>
          <div className="audit-stat-value">{filteredLogs.length}</div>
        </div>
        <div className="audit-stat">
          <div className="filter-label">Success</div>
          <div className="audit-stat-value" style={{ color: '#00ff00' }}>
            {filteredLogs.filter(l => l.status === 'success').length}
          </div>
        </div>
        <div className="audit-stat">
          <div className="filter-label">Failures</div>
          <div className="audit-stat-value" style={{ color: '#ff0000' }}>
            {filteredLogs.filter(l => l.status === 'failure').length}
          </div>
        </div>
      </div>

      {loading && filteredLogs.length === 0 ? (
        <div className="empty-state">Loading audit logs...</div>
      ) : filteredLogs.length === 0 ? (
        <div className="empty-state">No audit logs found</div>
      ) : (
        <div className="audit-log-list">
          {filteredLogs.map(log => (
            <div
              key={log.id}
              className={`audit-log-entry ${expandedId === log.id ? 'expanded' : ''}`}
              onClick={() => setExpandedId(expandedId === log.id ? null : log.id)}
            >
              <div className="log-entry-header">
                <span className="log-action-icon">{getActionIcon(log.action)}</span>
                <span className="log-actor">{log.actor_id}</span>
                <span className="log-resource">{log.resource_type}:{log.resource_id}</span>
                <span className="log-status" style={{ color: getStatusColor(log.status) }}>
                  {log.status}
                </span>
                <span className="log-timestamp">{new Date(log.timestamp).toLocaleTimeString()}</span>
              </div>
              {expandedId === log.id && (
                <div className="log-details">
                  <div><strong>Action:</strong> {log.action}</div>
                  <div><strong>Resource:</strong> {log.resource_type} ({log.resource_id})</div>
                  <div><strong>Details:</strong> {log.details}</div>
                  <div><strong>Timestamp:</strong> {new Date(log.timestamp).toLocaleString()}</div>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default AuditViewer;
