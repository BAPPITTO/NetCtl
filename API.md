# NetCtl API Documentation

Complete REST API reference for NetCtl Network Control Engine.

## Base URL

```
http://localhost:3001
```

## Authentication

Currently, NetCtl uses localhost-only access. Authentication (JWT) is planned for future releases.

## Response Format

All API responses follow this format:

```json
{
  "success": true,
  "data": { /* endpoint-specific data */ },
  "error": null
}
```

On error:
```json
{
  "success": false,
  "data": null,
  "error": "Error message"
}
```

## Endpoints

### System

#### Health Check
```
GET /api/health
```

Check if the daemon is running and responsive.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy"
  }
}
```

#### Get System State
```
GET /api/state
```

Retrieve complete system state including devices, VLANs, and configuration.

**Response:**
```json
{
  "success": true,
  "data": {
    "devices": {
      "device-id-1": {
        "id": "device-id-1",
        "mac": "aa:bb:cc:dd:ee:ff",
        "name": "WorkStation",
        "vlan_id": 10,
        "rate_limit_mbps": 100,
        "blocked": false,
        "created_at": "2026-04-08T14:30:00Z",
        "last_seen": "2026-04-08T14:45:00Z"
      }
    },
    "vlans": {
      "10": {
        "id": 10,
        "name": "Guest Network",
        "subnet": "192.168.10.0/24",
        "gateway": "192.168.10.1",
        "dhcp_enabled": true,
        "interface": "eth0.10",
        "created_at": "2026-04-08T14:20:00Z"
      }
    },
    "ipv4_forwarding_enabled": true,
    "xdp_attached": ["eth0"],
    "timestamp": "2026-04-08T14:45:00Z"
  }
}
```

### Interfaces

#### List Interfaces
```
GET /api/interfaces
```

Get list of available network interfaces with auto-detection.

**Response:**
```json
{
  "success": true,
  "data": {
    "interfaces": ["eth0", "eth1", "wlan0"]
  }
}
```

### VLANs

#### Create VLAN
```
POST /api/vlan
Content-Type: application/json
```

Create a new VLAN with automatic interface provisioning.

**Request:**
```json
{
  "vlan_id": 10,
  "name": "Guest Network",
  "subnet": "192.168.10.0/24",
  "gateway": "192.168.10.1",
  "dhcp_enabled": true
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 10,
    "name": "Guest Network",
    "subnet": "192.168.10.0/24",
    "gateway": "192.168.10.1",
    "dhcp_enabled": true,
    "dhcp_start": "192.168.10.100",
    "dhcp_end": "192.168.10.200",
    "interface": "eth0.10",
    "created_at": "2026-04-08T14:20:00Z"
  }
}
```

**Parameters:**
- `vlan_id` (number, required): VLAN ID (1-4094)
- `name` (string, required): Human-readable name
- `subnet` (string, required): Network in CIDR notation (e.g., 192.168.10.0/24)
- `gateway` (string, required): Gateway IP address
- `dhcp_enabled` (boolean, required): Whether to enable DHCP

#### List VLANs
```
GET /api/vlans
```

Get all configured VLANs.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 10,
      "name": "Guest Network",
      "subnet": "192.168.10.0/24",
      "gateway": "192.168.10.1",
      "dhcp_enabled": true,
      "interface": "eth0.10",
      "created_at": "2026-04-08T14:20:00Z"
    }
  ]
}
```

#### Delete VLAN
```
DELETE /api/vlan/:vlan_id
```

Remove a VLAN and clean up associated configuration.

**Parameters:**
- `vlan_id` (number, URL parameter): VLAN ID to delete

**Response:**
```json
{
  "success": true,
  "data": {
    "vlan_id": 10,
    "deleted": true
  }
}
```

### Devices

#### List Devices
```
GET /api/devices
```

Get all connected and managed devices.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "device-1",
      "mac": "aa:bb:cc:dd:ee:ff",
      "name": "WorkStation",
      "vlan_id": 10,
      "rate_limit_mbps": 100,
      "blocked": false,
      "created_at": "2026-04-08T14:30:00Z",
      "last_seen": "2026-04-08T14:45:00Z"
    }
  ]
}
```

#### Create Device
```
POST /api/devices
Content-Type: application/json
```

Register a new device.

**Request:**
```json
{
  "mac": "aa:bb:cc:dd:ee:ff",
  "name": "WorkStation",
  "vlan_id": 10
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "device-1",
    "mac": "aa:bb:cc:dd:ee:ff",
    "name": "WorkStation",
    "vlan_id": 10,
    "rate_limit_mbps": null,
    "blocked": false,
    "created_at": "2026-04-08T14:30:00Z",
    "last_seen": null
  }
}
```

**Parameters:**
- `mac` (string, required): MAC address (aa:bb:cc:dd:ee:ff format)
- `name` (string, required): Device name
- `vlan_id` (number, optional): VLAN ID to assign to device

### QoS (Quality of Service)

#### Set QoS Rule
```
POST /api/qos/device/:mac
Content-Type: application/json
```

Set rate limit or blocking for a device.

**Parameters:**
- `mac` (string, URL parameter): Device MAC address

**Request:**
```json
{
  "mac": "aa:bb:cc:dd:ee:ff",
  "rate_mbps": 100
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "mac": "aa:bb:cc:dd:ee:ff",
    "rate_mbps": 100
  }
}
```

**Special Values:**
- `rate_mbps: 0` - Block all traffic from device
- `rate_mbps: 100` - Limit to 100 Mbps
- `rate_mbps: 1000` - Limit to 1 Gbps

#### List QoS Rules
```
GET /api/qos/devices
```

Get all configured QoS rules.

**Response:**
```json
{
  "success": true,
  "data": {
    "aa:bb:cc:dd:ee:ff": 100,
    "11:22:33:44:55:66": 50,
    "ff:ff:ff:ff:ff:ff": 0
  }
}
```

#### Remove QoS Rule
```
DELETE /api/qos/device/:mac
```

Delete QoS rule for a device (remove rate limit).

**Parameters:**
- `mac` (string, URL parameter): Device MAC address

**Response:**
```json
{
  "success": true,
  "data": {
    "mac": "aa:bb:cc:dd:ee:ff",
    "removed": true
  }
}
```

### Metrics

#### Get Metrics Summary
```
GET /api/metrics/summary
```

Get aggregated system metrics.

**Response:**
```json
{
  "success": true,
  "data": {
    "devices": 5,
    "total_rate_mbps": 350.5,
    "packets_dropped": 1234,
    "total_bytes_sent": 1073741824,
    "timestamp": "2026-04-08T14:45:00Z"
  }
}
```

#### Stream Live Metrics (SSE)
```
GET /api/metrics/stream
```

Server-Sent Events stream for real-time metrics. This endpoint streams updates every ~100ms.

**Connection:**
```javascript
const eventSource = new EventSource('/api/metrics/stream');

eventSource.onmessage = (event) => {
  const metrics = JSON.parse(event.data);
  console.log(metrics);
};
```

**Data Format:**
```json
{
  "mac": "aa:bb:cc:dd:ee:ff",
  "packets_sent": 54321,
  "packets_dropped": 12,
  "bytes_sent": 536870912,
  "bytes_received": 268435456,
  "current_rate_mbps": 85.5,
  "timestamp": "2026-04-08T14:45:00.123Z"
}
```

## Error Responses

### 400 Bad Request
```json
{
  "success": false,
  "data": null,
  "error": "Invalid VLAN ID"
}
```

### 500 Internal Server Error
```json
{
  "success": false,
  "data": null,
  "error": "Failed to create VLAN: permission denied"
}
```

## Rate Limiting

Currently not enforced (coming in v0.2.0). All requests are accepted.

## Examples

### Create Guest Network VLAN
```bash
curl -X POST http://localhost:3001/api/vlan \
  -H "Content-Type: application/json" \
  -d '{
    "vlan_id": 20,
    "name": "Guest Network",
    "subnet": "192.168.20.0/24",
    "gateway": "192.168.20.1",
    "dhcp_enabled": true
  }'
```

### Add Device and Set Rate Limit
```bash
# Create device
curl -X POST http://localhost:3001/api/devices \
  -H "Content-Type: application/json" \
  -d '{
    "mac": "aa:bb:cc:dd:ee:ff",
    "name": "Video Streaming Device",
    "vlan_id": 20
  }'

# Set 50 Mbps limit for video streaming
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{
    "mac": "aa:bb:cc:dd:ee:ff",
    "rate_mbps": 50
  }'
```

### Block a Device
```bash
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{
    "mac": "aa:bb:cc:dd:ee:ff",
    "rate_mbps": 0
  }'
```

### Get Real-Time Metrics
```bash
# Python example
import requests
import json
from sseclient import SSEClient

url = 'http://localhost:3001/api/metrics/stream'
client = SSEClient(url)

for event in client:
    if event.data:
        metrics = json.loads(event.data)
        print(f"Device {metrics['mac']}: {metrics['current_rate_mbps']} Mbps")
```

## Pagination

Not currently implemented. All responses return complete data.

## Versioning

API version: `1.0` (v0.1.0 release)

Backward compatibility: All v0.x releases maintain API compatibility.

## Deprecation Policy

Deprecated endpoints will be marked 3 releases before removal.

## Rate Limiting

Coming in v0.2.0:
- 100 requests/second per IP
- 1000 requests/second globally
- Configurable via environment variables
