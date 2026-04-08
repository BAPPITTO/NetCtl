/// LAN Dashboard Configuration Component
/// Allows users to configure hostname, HTTPS, and verify DNS settings

import React, { useState, useEffect } from 'react';
import './LANConfig.css';

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
  const [localIP, setLocalIP] = useState('');

  useEffect(() => {
    // Detect local IP address
    detectLocalIP();
  }, []);

  const detectLocalIP = async () => {
    try {
      const response = await fetch('/api/network/local-ip');
      if (response.ok) {
        const data = await response.json();
        setLocalIP(data.ip_address);
        setConfig(prev => ({
          ...prev,
          local_ip_address: data.ip_address
        }));
      }
    } catch (error) {
      console.error('Failed to detect local IP:', error);
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
        setMessage({
          type: 'error',
          text: '⚠ DNS loop detected! The hostname resolves back to the dashboard IP.',
        });
      } else if (data.status === 'Valid') {
        setMessage({
          type: 'success',
          text: '✓ DNS verification successful!',
        });
      } else {
        setMessage({
          type: 'error',
          text: `✗ DNS verification failed: ${data.message}`,
        });
      }
    } catch (error) {
      console.error('DNS verification error:', error);
      setMessage({
        type: 'error',
        text: 'Failed to verify DNS configuration',
      });
    } finally {
      setIsVerifying(false);
    }
  };

  const saveConfiguration = async () => {
    setIsSaving(true);
    
    try {
      const response = await fetch('/api/dashboard/configure', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });

      if (response.ok) {
        const data = await response.json();
        setMessage({
          type: 'success',
          text: '✓ Dashboard configuration saved successfully!',
        });
      } else {
        setMessage({
          type: 'error',
          text: 'Failed to save configuration',
        });
      }
    } catch (error) {
      console.error('Configuration error:', error);
      setMessage({
        type: 'error',
        text: 'Failed to save dashboard configuration',
      });
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="lan-config-container">
      <div className="lan-config-header">
        <h2>LAN Dashboard Configuration</h2>
        <p>Configure how your NetCtl dashboard is accessed on the local network</p>
      </div>

      {message && (
        <div className={`message message-${message.type}`}>
          {message.text}
        </div>
      )}

      <div className="config-section">
        <h3>Dashboard Access Settings</h3>
        
        <div className="form-group">
          <label htmlFor="hostname">Hostname (FQDN)</label>
          <input
            type="text"
            id="hostname"
            name="hostname"
            value={config.hostname}
            onChange={handleInputChange}
            placeholder="e.g., netctl.local"
            className="form-control"
          />
          <small>The hostname used to access your dashboard on the LAN</small>
        </div>

        <div className="form-group">
          <label htmlFor="port">Port</label>
          <input
            type="number"
            id="port"
            name="port"
            value={config.port}
            onChange={handleInputChange}
            min="1024"
            max="65535"
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
            <span>Enable HTTPS</span>
          </label>
          <small>Uses self-signed certificate for secure LAN access</small>
        </div>

        <div className="form-group">
          <label htmlFor="dns_domain">DNS Domain</label>
          <input
            type="text"
            id="dns_domain"
            name="dns_domain"
            value={config.dns_domain}
            onChange={handleInputChange}
            placeholder="e.g., local"
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
            placeholder="Auto-detected"
          />
          <small>Automatically detected network IP</small>
        </div>
      </div>

      <div className="config-section">
        <h3>DNS Verification</h3>
        
        <button
          onClick={verifyDNS}
          disabled={isVerifying || !config.hostname}
          className="btn btn-secondary"
        >
          {isVerifying ? 'Verifying...' : 'Verify DNS Configuration'}
        </button>

        {dnsResults && (
          <div className={`dns-result dns-result-${dnsResults.status.toLowerCase()}`}>
            <h4>DNS Verification Result</h4>
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
                  <span className="warning-icon">⚠</span>
                  <span>DNS Loop Detected - hostname resolves back to dashboard</span>
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

      <div className="config-section">
        <h3>HTTPS Certificate</h3>
        
        {config.enable_https && (
          <div className="https-info">
            <div className="info-box">
              <h4>Self-Signed Certificate</h4>
              <p>A self-signed HTTPS certificate will be generated for secure LAN access.</p>
              <ul>
                <li>Certificate CN: {config.hostname}</li>
                <li>Validity: 365 days</li>
                <li>Key Size: 2048-bit RSA</li>
                <li>Browser Warning: Expected (self-signed)</li>
              </ul>
            </div>
          </div>
        )}
      </div>

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
        <h4>Setup Tips</h4>
        <ul>
          <li>Use a meaningful hostname like "netctl.local" for easy discovery</li>
          <li>Verify DNS is working before accessing the dashboard</li>
          <li>Browser warnings about self-signed certificates are normal</li>
          <li>Add hostname to your /etc/hosts if mDNS is not available</li>
          <li>Use HTTPS for secure communication on untrusted networks</li>
        </ul>
      </div>
    </div>
  );
};

export default LANConfigComponent;
