import React, { useEffect, useState, useCallback } from 'react';
import { getQosRules, setQosRule, removeQosRule } from '../api';

export default function QoSPanel() {
  const [rules, setRules] = useState<Record<string, number>>({});
  const [mac, setMac] = useState('');
  const [rate, setRate] = useState('');
  const [errors, setErrors] = useState<{ mac?: string; rate?: string }>({});

  const fetchRules = useCallback(async () => {
    try {
      const response = await getQosRules();
      setRules(response.data);
    } catch (error) {
      console.error('Failed to fetch QoS rules:', error);
    }
  }, []);

  useEffect(() => {
    fetchRules();
    const interval = setInterval(fetchRules, 3000);
    return () => clearInterval(interval);
  }, [fetchRules]);

  // Validate inputs
  useEffect(() => {
    const newErrors: typeof errors = {};
    const macRegex = /^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$/;
    if (!macRegex.test(mac)) newErrors.mac = 'Invalid MAC format';
    if (!rate || isNaN(Number(rate)) || Number(rate) < 0) newErrors.rate = 'Rate must be ≥ 0';
    setErrors(newErrors);
  }, [mac, rate]);

  const handleSetRule = async (e: React.FormEvent) => {
    e.preventDefault();
    if (Object.keys(errors).length > 0) return;

    try {
      await setQosRule(mac, Number(rate));
      setMac('');
      setRate('');
      fetchRules();
    } catch (error) {
      console.error('Failed to set QoS rule:', error);
    }
  };

  const handleRemoveRule = async (macAddr: string) => {
    try {
      await removeQosRule(macAddr);
      fetchRules();
    } catch (error) {
      console.error('Failed to remove QoS rule:', error);
    }
  };

  const inputStyle = (field: 'mac' | 'rate') => ({
    border: `1px solid ${errors[field] ? '#ff0000' : '#00ff00'}`,
    background: '#001100',
    color: '#00ff00',
    padding: '6px 10px',
    borderRadius: '4px',
    fontFamily: 'monospace',
    width: '100%',
  });

  const buttonDisabled = Object.keys(errors).length > 0 || !mac || !rate;

  return (
    <section style={{ padding: '20px', background: '#010101', borderRadius: '6px', border: '1px solid #00ff0033', color: '#00ff00', fontFamily: 'monospace' }}>
      <h2>Quality of Service (QoS)</h2>

      <form onSubmit={handleSetRule} style={{ marginBottom: '20px' }}>
        <div style={{ marginBottom: '10px' }}>
          <label>MAC Address</label>
          <input
            type="text"
            placeholder="aa:bb:cc:dd:ee:ff"
            value={mac}
            onChange={(e) => setMac(e.target.value)}
            style={inputStyle('mac')}
          />
          {errors.mac && <div style={{ color: '#ff4444', fontSize: '0.8em', marginTop: '2px' }}>{errors.mac}</div>}
        </div>

        <div style={{ marginBottom: '10px' }}>
          <label>Rate Limit (Mbps)</label>
          <input
            type="number"
            placeholder="100"
            value={rate}
            onChange={(e) => setRate(e.target.value)}
            style={inputStyle('rate')}
          />
          {errors.rate && <div style={{ color: '#ff4444', fontSize: '0.8em', marginTop: '2px' }}>{errors.rate}</div>}
        </div>

        <button type="submit" disabled={buttonDisabled} style={{
          background: buttonDisabled ? '#00220044' : '#00ff0022',
          border: '1px solid #00ff0044',
          padding: '8px 16px',
          borderRadius: '4px',
          cursor: buttonDisabled ? 'not-allowed' : 'pointer',
        }}>
          Set Rule
        </button>
      </form>

      <div>
        <h3>Active Rules</h3>
        <table style={{ width: '100%', borderCollapse: 'collapse', background: '#001100', borderRadius: '6px' }}>
          <thead>
            <tr>
              <th style={{ borderBottom: '1px solid #00ff0033', textAlign: 'left', padding: '8px' }}>MAC Address</th>
              <th style={{ borderBottom: '1px solid #00ff0033', textAlign: 'left', padding: '8px' }}>Rate Limit</th>
              <th style={{ borderBottom: '1px solid #00ff0033', textAlign: 'left', padding: '8px' }}>Action</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(rules).length === 0 ? (
              <tr>
                <td colSpan={3} style={{ textAlign: 'center', padding: '20px', opacity: 0.6 }}>No QoS rules configured</td>
              </tr>
            ) : (
              Object.entries(rules).map(([macAddr, rateLimit]) => (
                <tr key={macAddr}>
                  <td style={{ fontFamily: 'monospace', padding: '6px' }}>{macAddr}</td>
                  <td style={{ padding: '6px' }}>{rateLimit === 0 ? 'BLOCKED' : `${rateLimit} Mbps`}</td>
                  <td style={{ padding: '6px' }}>
                    <button
                      onClick={() => handleRemoveRule(macAddr)}
                      style={{
                        background: '#ff000055',
                        border: '1px solid #ff000077',
                        color: '#ff0000',
                        padding: '4px 8px',
                        borderRadius: '4px',
                        cursor: 'pointer'
                      }}
                    >
                      Remove
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}