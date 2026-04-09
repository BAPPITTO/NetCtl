import React, { useState, useEffect } from 'react';
import './LanConfig.css';

interface DNSVerificationResult {
  hostname: string;
  status: 'Valid' | 'Invalid' | 'Loopback' | 'Unreachable' | 'Misconfigured';
  resolved_ip?: string;
  expected_ip: string;
  loop_detected: boolean;
  message: string;
}

interface DashboardConfig {
  hostname: string;
  port: number;
  enable_https: boolean;
  dns_domain: string;
  local_ip_address: string;
}

const LANConfigComponent: React.FC = () => {
  const [config, setConfig] = useState<DashboardConfig>({
    hostname: 'netctl.local',
    port: 443,
    enable_https: true,
    dns_domain: 'local',
    local_ip_address: '',
  });

  const [dnsResults, setDNSResults] = useState<DNSVerificationResult | null>(null);
  const [isVerifying, setIsVerifying] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  useEffect(() => {
    detectLocalIP();
  }, []);

  const detectLocalIP = async () => {
    try {
      const response = await fetch('/api/network/local-ip');
      if (response.ok) {
        const data = await response.json();
        setConfig(prev => ({ ...prev, local_ip_address: data.ip_address }));
      }
    } catch {
      setMessage({ type: 'error', text: 'Failed to detect local IP address' });
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    const inputValue = type === 'checkbox' ? (e.target as HTMLInputElement).checked : value;

    setConfig(prev => ({
      ...prev,
      [name]: name === 'port' ? parseInt(value) : inputValue,
    }));
  };

  const verifyDNS = async () => {
    setIsVerifying(true);
    setDNSResults(null);
    setMessage(null);

    try {
      const response = await fetch('/api/dns/verify', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          hostname: config.hostname,
          expected_ip: config.local_ip_address,
        }),
      });

      const data = await response.json();
      setDNSResults(data);

      if (data.loop_detected) {
        setMessage({ type: 'error', text: 'DNS loop detected: hostname resolves back to dashboard IP' });
      } else if (data.status === 'Valid') {
        setMessage({ type: 'success', text: 'DNS verification successful' });
      } else {
        setMessage({ type: 'error', text: `DNS verification failed: ${data.message}` });
      }
    } catch {
      setMessage({ type: 'error', text: 'Error verifying DNS configuration' });
    } finally {
      setIsVerifying(false);
    }
  };

  const saveConfiguration = async () => {
    setIsSaving(true);
    setMessage(null);

    try {
      const response = await fetch('/api/dashboard/configure', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      if (response.ok) {
        setMessage({ type: 'success', text: 'Configuration saved successfully' });
      } else {
        setMessage({ type: 'error', text: 'Failed to save configuration' });
      }
    } catch {
      setMessage({ type: 'error', text: 'Error saving configuration' });
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="lan-config-container">
      <div className="lan-config-header">
        <h2>LAN Dashboard Configuration</h2>
        <p>Configure local network access for your NetCtl dashboard</p>
      </div>

      {message && (
        <div className={`message message-${message.type}`}>
          {message.text}
        </div>
      )}

      <div className="config-section">
        <h3>Dashboard Access</h3>
        <div className="form-group">
          <label htmlFor="hostname">Hostname</label>
          <input
            type="text"
            id="hostname"
            name="hostname"
            value={config.hostname}
            onChange={handleInputChange}
            className="form-control"
          />
          <small>Hostname for LAN access</small>
        </div>

        <div className="form-group">
          <label htmlFor="port">Port</label>
          <input
            type="number"
            id="port"
            name="port"
            value={config.port}
            onChange={handleInputChange}
            min={1024}
            max={65535}
            className="form-control"
          />
          <small>HTTPS port (443 recommended)</small>
        </div>

        <div className="form-group checkbox-group">
          <label htmlFor="enable_https">
            <input
              type="checkbox"
              id="enable_https"
              name="enable_https"
              checked={config.enable_https}
              onChange={handleInputChange}
              className="form-checkbox"
            />
            Enable HTTPS
          </label>
          <small>Self-signed certificate for secure access</small>
        </div>

        <div className="form-group">
          <label htmlFor="dns_domain">DNS Domain</label>
          <input
            type="text"
            id="dns_domain"
            name="dns_domain"
            value={config.dns_domain}
            onChange={handleInputChange}
            className="form-control"
          />
          <small>Local domain for mDNS/DNS resolution</small>
        </div>

        <div className="form-group">
          <label>Local IP Address</label>
          <input
            type="text"
            value={config.local_ip_address}
            disabled
            className="form-control disabled"
          />
          <small>Auto-detected IP</small>
        </div>
      </div>

      <div className="config-section">
        <h3>DNS Verification</h3>
        <button
          onClick={verifyDNS}
          disabled={isVerifying || !config.hostname}
          className="btn btn-secondary"
        >
          {isVerifying ? 'Verifying...' : 'Verify DNS'}
        </button>

        {dnsResults && (
          <div className={`dns-result dns-result-${dnsResults.status.toLowerCase()}`}>
            <h4>DNS Result</h4>
            <div className="result-details">
              <div className="result-row">
                <span className="label">Hostname:</span>
                <span className="value">{dnsResults.hostname}</span>
              </div>
              <div className="result-row">
                <span className="label">Status:</span>
                <span className={`status status-${dnsResults.status.toLowerCase()}`}>
                  {dnsResults.status}
                </span>
              </div>
              {dnsResults.resolved_ip && (
                <div className="result-row">
                  <span className="label">Resolved IP:</span>
                  <span className="value">{dnsResults.resolved_ip}</span>
                </div>
              )}
              <div className="result-row">
                <span className="label">Expected IP:</span>
                <span className="value">{dnsResults.expected_ip}</span>
              </div>
              {dnsResults.loop_detected && (
                <div className="result-row warning">
                  <span>DNS loop detected</span>
                </div>
              )}
              <div className="result-row">
                <span className="label">Message:</span>
                <span className="value">{dnsResults.message}</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {config.enable_https && (
        <div className="config-section">
          <h3>HTTPS Certificate</h3>
          <div className="https-info">
            <div className="info-box">
              <h4>Self-Signed Certificate</h4>
              <p>Secure LAN access with a self-signed certificate.</p>
              <ul>
                <li>CN: {config.hostname}</li>
                <li>Validity: 365 days</li>
                <li>Key: 2048-bit RSA</li>
                <li>Browser warnings are expected</li>
              </ul>
            </div>
          </div>
        </div>
      )}

      <div className="config-actions">
        <button
          onClick={saveConfiguration}
          disabled={isSaving}
          className="btn btn-primary"
        >
          {isSaving ? 'Saving...' : 'Save Configuration'}
        </button>
      </div>

      <div className="config-help">
        <h4>Tips</h4>
        <ul>
          <li>Use a clear hostname like "netctl.local"</li>
          <li>Verify DNS before accessing the dashboard</li>
          <li>Self-signed certificate warnings are normal</li>
          <li>Add hostname to /etc/hosts if needed</li>
          <li>Use HTTPS on untrusted networks</li>
        </ul>
      </div>
    </div>
  );
};

export default LANConfigComponent;