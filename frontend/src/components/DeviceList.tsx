import React, { useEffect, useState } from 'react';
import { Device } from '../api';
import './Dashboard.css'; // Reuse dashboard styles for Matrix theme

interface DeviceListProps {
  devices: Record<string, Device>;
}

export default function DeviceList({ devices }: DeviceListProps) {
  const [deviceList, setDeviceList] = useState<Device[]>([]);

  useEffect(() => {
    setDeviceList(Object.values(devices));
  }, [devices]);

  return (
    <section className="content-panel">
      <h2>Connected Devices</h2>
      {deviceList.length === 0 ? (
        <div className="empty-state">No devices connected</div>
      ) : (
        <div className="device-table">
          <table>
            <thead>
              <tr>
                <th>Device Name</th>
                <th>MAC Address</th>
                <th>VLAN ID</th>
                <th>Rate Limit</th>
                <th>Status</th>
                <th>Last Seen</th>
              </tr>
            </thead>
            <tbody>
              {deviceList.map((device) => (
                <tr key={device.id}>
                  <td>{device.name}</td>
                  <td className="mono">{device.mac}</td>
                  <td>{device.vlan_id ?? '-'}</td>
                  <td>{device.rate_limit_mbps ? `${device.rate_limit_mbps} Mbps` : '-'}</td>
                  <td>
                    <span className={`status ${device.blocked ? 'blocked' : 'active'}`}>
                      {device.blocked ? 'BLOCKED' : 'ACTIVE'}
                    </span>
                  </td>
                  <td>{device.last_seen ?? 'Never'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
}