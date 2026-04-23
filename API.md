# NetCtl API Documentation

Complete REST API reference for NetCtl Network Control Engine.

---

## Base URL

<http://localhost:3001>

---

## Authentication

Currently, NetCtl uses **localhost-only access**. Authentication (JWT) is planned for future releases. For now, ensure your API calls originate from the same machine for security.

---

## Response Format

**Success Response:**

```json
{
  "success": true,
  "data": { /* endpoint-specific data */ },
  "error": null
}

Error Response:

{
  "success": false,
  "data": null,
  "error": "Error message"
}


---

Endpoints

System

Health Check

GET /api/health
Check if the daemon is running and responsive.

Response:

{
  "success": true,
  "data": { "status": "healthy" }
}

Get System State

GET /api/state
Retrieve complete system state including devices, VLANs, and configuration.

Response:

{
  "success": true,
  "data": {
    "devices": { ... },
    "vlans": { ... },
    "ipv4_forwarding_enabled": true,
    "xdp_attached": ["eth0"],
    "timestamp": "2026-04-08T14:45:00Z"
  }
}


---

Interfaces

List Interfaces

GET /api/interfaces
Get list of available network interfaces.

Response:

{
  "success": true,
  "data": { "interfaces": ["eth0", "eth1", "wlan0"] }
}


---

VLANs

Create VLAN

POST /api/vlan
Create a new VLAN with automatic interface provisioning.

Request Body:

{
  "vlan_id": 10,
  "name": "Guest Network",
  "subnet": "192.168.10.0/24",
  "gateway": "192.168.10.1",
  "dhcp_enabled": true
}

Response:

{
  "success": true,
  "data": { ... VLAN object ... }
}

Parameters:

Field Type Required Description

vlan_id number yes VLAN ID (1-4094)
name string yes Human-readable name
subnet string yes Network in CIDR notation
gateway string yes Gateway IP address
dhcp_enabled boolean yes Enable DHCP


List VLANs

GET /api/vlans
Get all configured VLANs.

Response: Array of VLAN objects.

Delete VLAN

DELETE /api/vlan/:vlan_id
Remove a VLAN.

Response:

{
  "success": true,
  "data": { "vlan_id": 10, "deleted": true }
}


---

Devices

List Devices

GET /api/devices
Get all connected and managed devices.

Create Device

POST /api/devices
Register a new device.

Request Body:

{
  "mac": "aa:bb:cc:dd:ee:ff",
  "name": "WorkStation",
  "vlan_id": 10
}

Response:

{
  "success": true,
  "data": { ... device object ... }
}

Parameters:

Field Type Required Description

mac string yes MAC address
name string yes Device name
vlan_id number no VLAN ID assignment



---

QoS (Quality of Service)

Set QoS Rule

POST /api/qos/device/:mac
Set rate limit or blocking for a device.

Request:

{
  "mac": "aa:bb:cc:dd:ee:ff",
  "rate_mbps": 100
}

Special Values:

rate_mbps Effect

0 Block all traffic
100 Limit to 100 Mbps
1000 Limit to 1 Gbps


List QoS Rules

GET /api/qos/devices
Get all configured QoS rules.

Remove QoS Rule

DELETE /api/qos/device/:mac
Remove rate limit for a device.


---

Metrics

Get Metrics Summary

GET /api/metrics/summary
Aggregated system metrics.

Stream Live Metrics (SSE)

GET /api/metrics/stream
Server-Sent Events stream for real-time metrics. Example:

from sseclient import SSEClient
import json

client = SSEClient('http://localhost:3001/api/metrics/stream')
for event in client:
    metrics = json.loads(event.data)
    print(metrics)


---

Dashboard Configuration

Configure Dashboard

POST /api/dashboard/configure
Configure dashboard hostname, port, and HTTPS settings.

Verify DNS Configuration

POST /api/dns/verify
Check DNS resolution and loops.

Generate Self-Signed Certificate

POST /api/certificate/generate
Generate self-signed HTTPS certificate for dashboard.


---

Error Responses

400 Bad Request


{ "success": false, "data": null, "error": "Invalid VLAN ID" }

500 Internal Server Error


{ "success": false, "data": null, "error": "Failed to create VLAN: permission denied" }


---

Examples

Create Guest VLAN

curl -X POST http://localhost:3001/api/vlan \
  -H "Content-Type: application/json" \
  -d '{ "vlan_id": 20, "name": "Guest Network", "subnet": "192.168.20.0/24", "gateway": "192.168.20.1", "dhcp_enabled": true }'

Add Device and Set Rate Limit

curl -X POST http://localhost:3001/api/devices -H "Content-Type: application/json" -d '{"mac":"aa:bb:cc:dd:ee:ff","name":"Video Device","vlan_id":20}'
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff -H "Content-Type: application/json" -d '{"mac":"aa:bb:cc:dd:ee:ff","rate_mbps":50}'

Block a Device

curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff -H "Content-Type: application/json" -d '{"mac":"aa:bb:cc:dd:ee:ff","rate_mbps":0}'

Get Real-Time Metrics (Python SSE)

from sseclient import SSEClient
import json
client = SSEClient('http://localhost:3001/api/metrics/stream')
for event in client:
    metrics = json.loads(event.data)
    print(metrics)


---

Notes

Pagination: Not implemented; all responses return complete data.

API version: 1.0 (v0.1.0 release)

Deprecated endpoints will be marked 3 releases before removal.

Rate limiting coming in v0.2.0: 100 req/sec per IP, 1000 req/sec globally.
