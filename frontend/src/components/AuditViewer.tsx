import React, { useEffect, useState, useMemo } from 'react';

interface AuditEntry {
  id: string;
  actor_id: string;
  action: string;
  resource_type: string;
  resource_id: string;
  status: 'success' | 'failure' | 'partial' | string;
  timestamp: string;
  details: string;
}

interface AuditViewerProps {
  apiUrl?: string;
  refreshInterval?: number;
}

type FilterBy = 'all' | 'actor' | 'action' | 'status';

const STATUS_COLORS: Record<string, string> = {
  success: '#00ff00',
  failure: '#ff0000',
  partial: '#ffff00',
};

const ACTION_LABELS: Record<string, string> = {
  create: 'CREATE',
  delete: 'DELETE',
  update: 'UPDATE',
  read: 'READ',
  login: 'LOGIN',
  logout: 'LOGOUT',
  export: 'EXPORT',
  configure: 'CONFIGURE',
  execute: 'EXECUTE',
};

const AuditViewer: React.FC<AuditViewerProps> = ({
  apiUrl = 'http://localhost:3001',
  refreshInterval = 10000,
}) => {
  const [logs, setLogs] = useState<AuditEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<FilterBy>('all');
  const [filterValue, setFilterValue] = useState('');
  const [expandedId, setExpandedId] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;

    const fetchLogs = async () => {
      try {
        setLoading(true);
        let url = `${apiUrl}/api/audit/logs?limit=100`;

        if (filterType === 'status' && filterValue) {
          url = `${apiUrl}/api/audit/logs/status/${encodeURIComponent(filterValue)}`;
        }

        const response = await fetch(url);
        const data = await response.json();

        if (!isMounted) return;

        if (data.success && Array.isArray(data.data?.logs)) {
          setLogs(data.data.logs);
          setError(null);
        } else {
          setError(data.error || 'Failed to fetch audit logs');
        }
      } catch (err) {
        if (!isMounted) return;
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        if (isMounted) setLoading(false);
      }
    };

    fetchLogs();
    const interval = setInterval(fetchLogs, refreshInterval);
    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, [apiUrl, filterType, filterValue, refreshInterval]);

  const filteredLogs = useMemo(() => {
    return logs.filter((log) => {
      if (filterType === 'status' && filterValue) {
        return log.status === filterValue;
      }
      if (filterType === 'actor' && filterValue) {
        return log.actor_id.toLowerCase().includes(filterValue.toLowerCase());
      }
      if (filterType === 'action' && filterValue) {
        return log.action.toLowerCase().includes(filterValue.toLowerCase());
      }
      return true;
    });
  }, [logs, filterType, filterValue]);

  const getStatusColor = (status: string) => STATUS_COLORS[status] ?? '#00ffff';
  const getActionLabel = (action: string) => ACTION_LABELS[action] ?? 'UNKNOWN';

  return (
    <div className="audit-viewer">
      <div className="audit-header">Audit Log Viewer</div>
      {error && <div className="error">{error}</div>}

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
            <option value="actor">Actor</option>
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
            {filteredLogs.filter((l) => l.status === 'success').length}
          </div>
        </div>
        <div className="audit-stat">
          <div className="filter-label">Failures</div>
          <div className="audit-stat-value" style={{ color: '#ff0000' }}>
            {filteredLogs.filter((l) => l.status === 'failure').length}
          </div>
        </div>
      </div>

      {loading && filteredLogs.length === 0 ? (
        <div className="empty-state">Loading audit logs...</div>
      ) : filteredLogs.length === 0 ? (
        <div className="empty-state">No audit logs found</div>
      ) : (
        <div className="audit-log-list">
          {filteredLogs.map((log) => (
            <div
              key={log.id}
              className={`audit-log-entry ${expandedId === log.id ? 'expanded' : ''}`}
              onClick={() => setExpandedId(expandedId === log.id ? null : log.id)}
            >
              <div className="log-entry-header">
                <span className="log-action-icon">{getActionLabel(log.action)}</span>
                <span className="log-actor">{log.actor_id}</span>
                <span className="log-resource">
                  {log.resource_type}:{log.resource_id}
                </span>
                <span className="log-status" style={{ color: getStatusColor(log.status) }}>
                  {log.status}
                </span>
                <span className="log-timestamp">
                  {new Date(log.timestamp).toLocaleTimeString()}
                </span>
              </div>
              {expandedId === log.id && (
                <div className="log-details">
                  <div>
                    <strong>Action:</strong> {log.action}
                  </div>
                  <div>
                    <strong>Resource:</strong> {log.resource_type} ({log.resource_id})
                  </div>
                  <div>
                    <strong>Details:</strong> {log.details}
                  </div>
                  <div>
                    <strong>Timestamp:</strong>{' '}
                    {new Date(log.timestamp).toLocaleString()}
                  </div>
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