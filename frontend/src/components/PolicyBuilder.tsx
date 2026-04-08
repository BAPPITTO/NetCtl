import React, { useState, useCallback } from 'react';

interface PolicyRule {
  id: string;
  name: string;
  priority: number;
  match_criteria: string;
  action: string;
}

interface PolicyBuilderProps {
  apiUrl?: string;
  onPolicySaved?: (policy: PolicyRule) => void;
}

const PolicyBuilder: React.FC<PolicyBuilderProps> = ({ 
  apiUrl = 'http://localhost:3001',
  onPolicySaved 
}) => {
  const [policies, setPolicies] = useState<PolicyRule[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [formData, setFormData] = useState({
    name: '',
    priority: 100,
    match_criteria: '',
    action: 'allow',
  });

  const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: name === 'priority' ? parseInt(value) || 0 : value,
    }));
  }, []);

  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!formData.name || !formData.match_criteria) {
      setError('Policy name and match criteria are required');
      return;
    }

    try {
      setLoading(true);
      const response = await fetch(`${apiUrl}/api/policies`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      const data = await response.json();

      if (data.success) {
        const newPolicy: PolicyRule = {
          id: data.data?.policy_id || `policy_${Date.now()}`,
          ...formData,
        };

        setPolicies(prev => [...prev, newPolicy]);
        setFormData({ name: '', priority: 100, match_criteria: '', action: 'allow' });
        setShowForm(false);
        setError(null);

        if (onPolicySaved) {
          onPolicySaved(newPolicy);
        }
      } else {
        setError(data.error || 'Failed to create policy');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, [formData, apiUrl, onPolicySaved]);

  const handleDelete = useCallback(async (policyId: string) => {
    try {
      const response = await fetch(`${apiUrl}/api/policies/${policyId}`, {
        method: 'DELETE',
      });

      if (response.ok) {
        setPolicies(prev => prev.filter(p => p.id !== policyId));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete policy');
    }
  }, [apiUrl]);

  return (
    <div className="policy-builder">
      <style>{`
        .policy-builder {
          padding: 20px;
          background: rgba(255, 0, 255, 0.05);
          border: 1px solid rgba(255, 0, 255, 0.2);
          border-radius: 4px;
          margin: 10px 0;
        }

        .policy-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          color: #ff00ff;
          font-weight: bold;
          margin-bottom: 15px;
          text-shadow: 0 0 10px rgba(255, 0, 255, 0.5);
        }

        .policy-btn {
          background: rgba(255, 0, 255, 0.2);
          border: 1px solid rgba(255, 0, 255, 0.4);
          color: #ff00ff;
          padding: 8px 16px;
          border-radius: 4px;
          cursor: pointer;
          font-family: monospace;
          transition: all 0.3s;
        }

        .policy-btn:hover {
          background: rgba(255, 0, 255, 0.3);
          border-color: rgba(255, 0, 255, 0.6);
          box-shadow: 0 0 10px rgba(255, 0, 255, 0.3);
        }

        .policy-form {
          background: rgba(255, 0, 255, 0.08);
          border: 1px solid rgba(255, 0, 255, 0.2);
          padding: 15px;
          border-radius: 4px;
          margin-bottom: 15px;
        }

        .form-group {
          margin-bottom: 12px;
        }

        .form-label {
          display: block;
          color: #ff00ff;
          font-weight: bold;
          margin-bottom: 5px;
          font-size: 0.9em;
        }

        .form-input,
        .form-textarea,
        .form-select {
          width: 100%;
          padding: 8px;
          background: rgba(0, 0, 0, 0.3);
          border: 1px solid rgba(255, 0, 255, 0.3);
          color: #ff00ff;
          border-radius: 4px;
          font-family: monospace;
          box-sizing: border-box;
        }

        .form-textarea {
          resize: vertical;
          min-height: 60px;
          font-family: monospace;
        }

        .form-input:focus,
        .form-textarea:focus,
        .form-select:focus {
          outline: none;
          border-color: rgba(255, 0, 255, 0.6);
          box-shadow: 0 0 10px rgba(255, 0, 255, 0.2);
        }

        .form-actions {
          display: flex;
          gap: 10px;
          margin-top: 15px;
        }

        .form-actions button {
          flex: 1;
          padding: 10px;
          background: rgba(255, 0, 255, 0.2);
          border: 1px solid rgba(255, 0, 255, 0.4);
          color: #ff00ff;
          border-radius: 4px;
          cursor: pointer;
          font-family: monospace;
          transition: all 0.3s;
        }

        .form-actions button:hover:not(:disabled) {
          background: rgba(255, 0, 255, 0.3);
          box-shadow: 0 0 10px rgba(255, 0, 255, 0.3);
        }

        .form-actions button:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .policy-list {
          margin-top: 15px;
        }

        .policy-item {
          background: rgba(255, 0, 255, 0.08);
          border-left: 3px solid rgba(255, 0, 255, 0.4);
          padding: 12px;
          margin: 8px 0;
          border-radius: 4px;
          display: flex;
          justify-content: space-between;
          align-items: center;
          color: #ff00ff;
          font-family: monospace;
        }

        .policy-item-info {
          flex: 1;
        }

        .policy-name {
          font-weight: bold;
          margin-bottom: 5px;
        }

        .policy-details {
          font-size: 0.8em;
          opacity: 0.8;
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
          gap: 10px;
        }

        .policy-delete-btn {
          background: rgba(255, 0, 0, 0.2);
          border: 1px solid rgba(255, 0, 0, 0.4);
          color: #ff0000;
          padding: 6px 12px;
          border-radius: 4px;
          cursor: pointer;
          margin-left: 10px;
          transition: all 0.3s;
        }

        .policy-delete-btn:hover {
          background: rgba(255, 0, 0, 0.3);
          box-shadow: 0 0 10px rgba(255, 0, 0, 0.3);
        }

        .error {
          background: rgba(255, 0, 0, 0.1);
          border: 1px solid rgba(255, 0, 0, 0.3);
          color: #ff0000;
          padding: 10px;
          border-radius: 4px;
          margin-bottom: 15px;
        }

        .empty-state {
          color: rgba(255, 0, 255, 0.5);
          text-align: center;
          padding: 20px;
          font-style: italic;
        }
      `}</style>

      <div className="policy-header">
        <span>🔐 Network Policy Manager</span>
        <button className="policy-btn" onClick={() => setShowForm(!showForm)}>
          {showForm ? '− Close' : '+ New Policy'}
        </button>
      </div>

      {error && <div className="error">⚠️ {error}</div>}

      {showForm && (
        <form className="policy-form" onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="form-label">Policy Name</label>
            <input
              className="form-input"
              type="text"
              name="name"
              placeholder="e.g., Block Torrents"
              value={formData.name}
              onChange={handleInputChange}
              required
            />
          </div>

          <div className="form-group">
            <label className="form-label">Priority (0-1000)</label>
            <input
              className="form-input"
              type="number"
              name="priority"
              min="0"
              max="1000"
              value={formData.priority}
              onChange={handleInputChange}
            />
          </div>

          <div className="form-group">
            <label className="form-label">Match Criteria</label>
            <textarea
              className="form-textarea"
              name="match_criteria"
              placeholder="e.g., dst_port >= 6881 AND dst_port <= 6999"
              value={formData.match_criteria}
              onChange={handleInputChange}
              required
            />
          </div>

          <div className="form-group">
            <label className="form-label">Action</label>
            <select
              className="form-select"
              name="action"
              value={formData.action}
              onChange={handleInputChange}
            >
              <option value="allow">Allow</option>
              <option value="block">Block</option>
              <option value="rate_limit">Rate Limit</option>
              <option value="shape">Traffic Shape</option>
            </select>
          </div>

          <div className="form-actions">
            <button type="submit" disabled={loading}>
              {loading ? 'Creating...' : 'Create Policy'}
            </button>
            <button
              type="button"
              onClick={() => setShowForm(false)}
              style={{ background: 'rgba(100, 100, 100, 0.2)', borderColor: 'rgba(100, 100, 100, 0.4)' }}
            >
              Cancel
            </button>
          </div>
        </form>
      )}

      <div className="policy-list">
        {policies.length === 0 ? (
          <div className="empty-state">No policies created yet. Add one to get started!</div>
        ) : (
          policies.map(policy => (
            <div key={policy.id} className="policy-item">
              <div className="policy-item-info">
                <div className="policy-name">{policy.name}</div>
                <div className="policy-details">
                  <span>Priority: {policy.priority}</span>
                  <span>Action: {policy.action}</span>
                  <span>Criteria: {policy.match_criteria.substring(0, 50)}...</span>
                </div>
              </div>
              <button
                className="policy-delete-btn"
                onClick={() => handleDelete(policy.id)}
              >
                Delete
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default PolicyBuilder;
