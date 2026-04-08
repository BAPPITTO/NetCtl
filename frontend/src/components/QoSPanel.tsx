import React, { useEffect, useState } from 'react';
import { getQosRules, setQosRule, removeQosRule } from '../api';

export default function QoSPanel() {
  const [rules, setRules] = useState<Record<string, number>>({});
  const [mac, setMac] = useState('');
  const [rate, setRate] = useState('');

  useEffect(() => {
    const fetchRules = async () => {
      try {
        const response = await getQosRules();
        setRules(response.data);
      } catch (error) {
        console.error('Failed to fetch QoS rules:', error);
      }
    };

    fetchRules();
    const interval = setInterval(fetchRules, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleSetRule = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await setQosRule(mac, parseInt(rate));
      setMac('');
      setRate('');
    } catch (error) {
      console.error('Failed to set QoS rule:', error);
    }
  };

  const handleRemoveRule = async (macAddr: string) => {
    try {
      await removeQosRule(macAddr);
    } catch (error) {
      console.error('Failed to remove QoS rule:', error);
    }
  };

  return (
    <section className="content-panel">
      <h2>Quality of Service (QoS)</h2>
      
      <form className="qos-form" onSubmit={handleSetRule}>
        <div className="form-group">
          <label>MAC Address</label>
          <input
            type="text"
            placeholder="aa:bb:cc:dd:ee:ff"
            value={mac}
            onChange={(e) => setMac(e.target.value)}
            required
          />
        </div>
        <div className="form-group">
          <label>Rate Limit (Mbps)</label>
          <input
            type="number"
            placeholder="100"
            value={rate}
            onChange={(e) => setRate(e.target.value)}
            required
          />
        </div>
        <button type="submit" className="btn-primary">Set Rule</button>
      </form>

      <div className="qos-rules">
        <h3>Active Rules</h3>
        <table>
          <thead>
            <tr>
              <th>MAC Address</th>
              <th>Rate Limit (Mbps)</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(rules).map(([macAddr, rateLimit]) => (
              <tr key={macAddr}>
                <td className="mono">{macAddr}</td>
                <td>{rateLimit === 0 ? 'BLOCKED' : `${rateLimit} Mbps`}</td>
                <td>
                  <button 
                    className="btn-delete"
                    onClick={() => handleRemoveRule(macAddr)}
                  >
                    Remove
                  </button>
                </td>
              </tr>
            ))}
            {Object.keys(rules).length === 0 && (
              <tr>
                <td colSpan={3} style={{ textAlign: 'center', padding: '20px' }}>
                  No QoS rules configured
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}
