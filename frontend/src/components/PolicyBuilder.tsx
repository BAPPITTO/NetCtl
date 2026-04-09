import React, { useState, useCallback, useEffect } from 'react';

interface PolicyRule {
  id: string;
  name: string;
  priority: number;
  match_criteria: string;
  action: 'allow' | 'block' | 'rate_limit' | 'shape';
}

interface PolicyBuilderProps {
  apiUrl?: string;
  onPolicySaved?: (policy: PolicyRule) => void;
}

const PolicyBuilder: React.FC<PolicyBuilderProps> = ({
  apiUrl = 'http://localhost:3001',
  onPolicySaved,
}) => {
  const [policies, setPolicies] = useState<PolicyRule[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [loading, setLoading] = useState(false);

  const [formData, setFormData] = useState<Omit<PolicyRule, 'id'>>({
    name: '',
    priority: 100,
    match_criteria: '',
    action: 'allow',
  });

  const [formErrors, setFormErrors] = useState<{ name?: string; priority?: string; match_criteria?: string }>({});

  // --- live validation ---
  useEffect(() => {
    const errors: typeof formErrors = {};
    if (!formData.name.trim()) errors.name = 'Name is required';
    if (!formData.match_criteria.trim()) errors.match_criteria = 'Match criteria is required';
    if (formData.priority < 0 || formData.priority > 1000) errors.priority = 'Priority must be 0-1000';
    setFormErrors(errors);
  }, [formData]);

  const updateForm = useCallback(
    (field: keyof typeof formData, value: string | number) => {
      setFormData((prev) => ({
        ...prev,
        [field]: field === 'priority' ? Number(value) : value,
      }));
    },
    []
  );

  const createPolicy = useCallback(async () => {
    if (Object.keys(formErrors).length > 0) return;
    setLoading(true);

    try {
      const response = await fetch(`${apiUrl}/api/policies`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });
      const data = await response.json();

      const newPolicy: PolicyRule = {
        id: data.data?.policy_id || `policy_${Date.now()}`,
        ...formData,
      };

      if (data.success) {
        setPolicies((prev) => [...prev, newPolicy]);
        setFormData({ name: '', priority: 100, match_criteria: '', action: 'allow' });
        setShowForm(false);
        onPolicySaved?.(newPolicy);
      }
    } catch (err) {
      console.error('Failed to create policy', err);
    } finally {
      setLoading(false);
    }
  }, [formData, formErrors, apiUrl, onPolicySaved]);

  const deletePolicy = useCallback(
    async (id: string) => {
      try {
        const response = await fetch(`${apiUrl}/api/policies/${id}`, { method: 'DELETE' });
        if (response.ok) setPolicies((prev) => prev.filter((p) => p.id !== id));
      } catch (err) {
        console.error('Failed to delete policy', err);
      }
    },
    [apiUrl]
  );

  const actionColor = (action: PolicyRule['action']) => {
    switch (action) {
      case 'allow':
        return '#00ff00';
      case 'block':
        return '#ff0000';
      case 'rate_limit':
        return '#ffff00';
      case 'shape':
        return '#00ffff';
      default:
        return '#ffffff';
    }
  };

  const inputBorder = (field: keyof typeof formErrors) =>
    formErrors[field] ? '1px solid #ff0000' : '1px solid #00ff00';

  return (
    <div className="policy-builder">
      <style>{`
        .policy-builder { padding:20px; background:#010101; border:1px solid #00ff0033; border-radius:6px; color:#00ff00; font-family:monospace; }
        .policy-header { display:flex; justify-content:space-between; align-items:center; margin-bottom:15px; font-weight:bold; }
        .policy-btn { background:#00ff0022; border:1px solid #00ff0044; padding:8px 16px; border-radius:4px; cursor:pointer; transition:.2s; }
        .policy-btn:hover { background:#00ff0033; box-shadow:0 0 12px #00ff0055; }
        .policy-form { background:#002200; border:1px solid #00ff0033; padding:15px; border-radius:6px; margin-bottom:15px; }
        .form-group { margin-bottom:12px; }
        .form-label { display:block; margin-bottom:5px; font-weight:bold; }
        .form-input, .form-textarea, .form-select { width:100%; padding:8px; border-radius:4px; color:#00ff00; background:#001100; font-family:monospace; box-sizing:border-box; }
        .form-textarea { min-height:60px; resize:vertical; }
        .form-actions { display:flex; gap:10px; margin-top:12px; }
        .form-actions button { flex:1; padding:10px; background:#00ff0022; border:1px solid #00ff0044; cursor:pointer; transition:.2s; }
        .form-actions button:hover:not(:disabled) { background:#00ff0033; box-shadow:0 0 8px #00ff0055; }
        .form-actions button:disabled { opacity:0.5; cursor:not-allowed; }
        .form-error { color:#ff4444; font-size:0.8em; margin-top:3px; }
        .policy-list { margin-top:15px; }
        .policy-item { display:flex; justify-content:space-between; align-items:center; padding:12px; border-left:3px solid #00ff0066; margin-bottom:10px; background:#001100; border-radius:6px; transition:.2s; }
        .policy-item:hover { box-shadow:0 0 10px #00ff0044; }
        .policy-item-info { flex:1; }
        .policy-name { font-weight:bold; margin-bottom:5px; }
        .policy-details { font-size:0.85em; display:grid; grid-template-columns:repeat(auto-fit, minmax(150px, 1fr)); gap:5px; opacity:0.85; }
        .policy-action-badge { font-weight:bold; padding:2px 6px; border-radius:4px; }
        .policy-delete-btn { background:#ff000055; border:1px solid #ff000077; padding:6px 12px; border-radius:4px; cursor:pointer; color:#ff0000; transition:.2s; }
        .policy-delete-btn:hover { background:#ff000077; box-shadow:0 0 8px #ff000088; }
        .empty-state { text-align:center; padding:20px; font-style:italic; opacity:0.6; }
      `}</style>

      <div className="policy-header">
        <span>🔐 Network Policy Manager</span>
        <button className="policy-btn" onClick={() => setShowForm(!showForm)}>
          {showForm ? '− Close' : '+ New Policy'}
        </button>
      </div>

      {showForm && (
        <div className="policy-form">
          <div className="form-group">
            <label className="form-label">Policy Name</label>
            <input
              className="form-input"
              style={{ border: inputBorder('name') }}
              value={formData.name}
              onChange={(e) => updateForm('name', e.target.value)}
              placeholder="e.g., Block Torrents"
            />
            {formErrors.name && <div className="form-error">{formErrors.name}</div>}
          </div>

          <div className="form-group">
            <label className="form-label">Priority</label>
            <input
              className="form-input"
              style={{ border: inputBorder('priority') }}
              type="number"
              value={formData.priority}
              min={0}
              max={1000}
              onChange={(e) => updateForm('priority', e.target.value)}
            />
            {formErrors.priority && <div className="form-error">{formErrors.priority}</div>}
          </div>

          <div className="form-group">
            <label className="form-label">Match Criteria</label>
            <textarea
              className="form-textarea"
              style={{ border: inputBorder('match_criteria') }}
              value={formData.match_criteria}
              onChange={(e) => updateForm('match_criteria', e.target.value)}
              placeholder="e.g., dst_port >= 6881 AND dst_port <= 6999"
            />
            {formErrors.match_criteria && <div className="form-error">{formErrors.match_criteria}</div>}
          </div>

          <div className="form-group">
            <label className="form-label">Action</label>
            <select
              className="form-select"
              value={formData.action}
              onChange={(e) => updateForm('action', e.target.value)}
            >
              <option value="allow">Allow</option>
              <option value="block">Block</option>
              <option value="rate_limit">Rate Limit</option>
              <option value="shape">Traffic Shape</option>
            </select>
          </div>

          <div className="form-actions">
            <button onClick={createPolicy} disabled={loading || Object.keys(formErrors).length > 0}>
              {loading ? 'Creating...' : 'Create Policy'}
            </button>
            <button onClick={() => setShowForm(false)} style={{ background: '#33333322', borderColor: '#33333344', color: '#ffffffaa' }}>
              Cancel
            </button>
          </div>
        </div>
      )}

      <div className="policy-list">
        {policies.length === 0 ? (
          <div className="empty-state">No policies created yet. Add one to get started!</div>
        ) : (
          policies.map((p) => (
            <div key={p.id} className="policy-item">
              <div className="policy-item-info">
                <div className="policy-name">{p.name}</div>
                <div className="policy-details">
                  <span>Priority: {p.priority}</span>
                  <span>
                    Action: <span className="policy-action-badge" style={{ background: actionColor(p.action), color: '#000' }}>{p.action}</span>
                  </span>
                  <span>Criteria: {p.match_criteria.length > 50 ? p.match_criteria.substring(0, 50) + '...' : p.match_criteria}</span>
                </div>
              </div>
              <button className="policy-delete-btn" onClick={() => deletePolicy(p.id)}>Delete</button>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default PolicyBuilder;