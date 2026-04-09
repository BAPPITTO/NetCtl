
# NetCtl - Advanced Network Control Engine

A production-grade network management daemon built in Rust with a TypeScript/React dashboard. NetCtl provides transactional state management, idempotent VLAN/DHCP configuration, eBPF/XDP kernel-level QoS, and live metrics streaming.

---

## Features

### 🖥️ Interactive Setup

- **TUI Setup Wizard** - Matrix-style terminal configuration with Ratatui framework
- **8-Screen Configuration Flow** - Network, DNS, dashboard, and admin setup
- **Real-time Validation** - IPv4, hostname, and DNS verification during setup
- **LAN Dashboard Configuration** - Hostname, DNS verification, HTTPS support
- **DNS Loop Detection** - Prevents configuration errors and circular dependencies

### 🧠 Core Architecture

- **Transactional State Management** - Every network change is reversible with full rollback
- **Idempotent Configuration** - Safe to reapply changes without duplication
- **Persistent State** - SQLite-backed state with versioning support
- **Async Event-Driven** - Tokio-based non-blocking networking control

### 🌐 Network Management

- **Interface Detection** - Automatic WAN/LAN interface detection
- **Multi-Interface Support** - Manages multiple NICs cleanly
- **IPv4 Forwarding Control** - Safe enablement/disablement
- **Link State Monitoring** - Real-time interface health tracking

### 🧩 VLAN System

- **Dynamic VLAN Creation** - 802.1Q tagging support
- **Per-VLAN Provisioning** - Automatic interface creation (eth0.10, etc.)
- **Subnet Assignment** - CIDR notation support
- **VLAN Isolation** - Complete traffic separation
- **DHCP Per-VLAN** - Independent DHCP scopes

### 📡 DHCP Management

- **Automatic Config Generation** - dnsmasq-backed DHCP
- **Dynamic Lease Allocation** - Range calculation from subnet
- **DHCP Service Management** - Automatic start/stop/restart
- **Conflict Avoidance** - Detection of existing DHCP services

### ⚡ QoS & Traffic Control

- **Per-Device Rate Limiting** - Bandwidth allocation by MAC
- **Per-Device Blocking** - Instant packet drop
- **eBPF/XDP Enforcement** - Kernel-space zero-copy filtering
- **Real-Time Updates** - No restart required
- **Device Prioritization** - Extendable priority system

### 📊 Live Metrics & Telemetry

- **Real-Time Streaming** - SSE-based live updates
- **Per-Device Statistics** - Traffic visibility
- **Packet Counters** - Kernel map exposure
- **Live Dashboard** - No refresh required

### 🖥️ Matrix Cyberpunk Dashboard

- **Cyberpunk Aesthetic** - Green-on-black Matrix theme
- **Live Device Stats** - Real-time traffic visibility
- **Inline Editing** - Modify devices and VLANs on-the-fly
- **Responsive Design** - Desktop and mobile support
- **Background Animation** - Matrix rain effect

### 🔌 RESTful API

- **State Retrieval** - Full system state endpoint
- **VLAN Management** - Create, read, delete VLANs
- **Device Management** - Add/remove devices
- **QoS Control** - Set and remove rate limits
- **Metrics Streaming** - SSE endpoint for live data
- **WoL Integration** - Wake-on-LAN triggers
- **DNS Verification** - Async DNS resolution with loop detection
- **Dashboard Configuration** - LAN hostname and HTTPS settings
- **Certificate Management** - Self-signed cert generation and validation

### 📦 Enterprise Deployment

- **Debian Packages** - .deb packages for Ubuntu/Debian systems
- **Red Hat Packages** - .rpm packages for CentOS/RHEL/Fedora
- **Systemd Integration** - Hardened service unit with security constraints
- **Multi-Package Split** - Separate daemon, CLI, and dashboard packages
- **Automated Build** - Single-command package generation script

### 🔄 Transaction & Rollback

- **Ordered Operations** - Sequential execution order
- **Failure Detection** - Mid-apply error handling
- **Reverse Rollback** - Automatic undo on failure
- **Partial Recovery** - Graceful state recovery
- **Operation Isolation** - Each change is independent

## Quick Start

### Prerequisites

- Linux kernel 5.8+ (for XDP support)
- Rust 1.70+ (for backend compilation)
- Node.js 18+ (for frontend)
- LLVM tools (for eBPF compilation)

### Build Backend

```bash
cd backend
cargo build --release
ls target/release/netctl-daemon

Build Frontend

cd frontend
npm install
npm run build
ls dist/

Run Daemon

# Start the backend (requires root for network operations)
sudo ./backend/target/release/netctl-daemon

# API available at http://localhost:3001

Run Setup Wizard

# Interactive TUI configuration on Linux
sudo cargo run --bin netctl-tui --manifest-path=backend/Cargo.toml

Run Dashboard (Development)

cd frontend
npm run dev
# Dashboard at http://localhost:5173
# API proxied from http://localhost:3001


---

API Endpoints

State Management

Method Endpoint Description

GET /api/state Get complete system state
GET /api/health Health check


Interfaces

| GET | /api/interfaces | List all interfaces |

VLANs

| POST | /api/vlan | Create new VLAN | | GET | /api/vlan | List all VLANs | | DELETE | /api/vlan/:vlan_id | Delete VLAN |

Devices

| GET | /api/devices | List devices | | POST | /api/devices | Create device |

QoS

| POST | /api/qos/device/:mac | Set rate limit | | DELETE | /api/qos/device/:mac | Remove rate limit | | GET | /api/qos/devices | List QoS rules |

Metrics

| GET | /api/metrics/summary | Aggregate metrics | | GET | /api/metrics/stream | SSE live stream |


---

Example Usage

Create a VLAN

curl -X POST http://localhost:3001/api/vlan \
  -H "Content-Type: application/json" \
  -d '{
    "vlan_id": 10,
    "name": "Guest Network",
    "subnet": "192.168.10.0/24",
    "gateway": "192.168.10.1",
    "dhcp_enabled": true
  }'

Set QoS Rule

curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{
    "mac": "aa:bb:cc:dd:ee:ff",
    "rate_mbps": 100
  }'

Block a Device

curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{
    "mac": "aa:bb:cc:dd:ee:ff",
    "rate_mbps": 0
  }'


---

Project Structure

netctl/
├── backend/ # Rust backend daemon
│ ├── Cargo.toml
│ ├── src/
│ │ ├── main.rs # Daemon entry point
│ │ ├── lib.rs # Library exports
│ │ ├── state.rs # Transactional state
│ │ ├── db.rs # SQLite integration
│ │ ├── network/ # Network operations
│ │ ├── api/ # REST API handlers
│ │ ├── qos.rs # QoS management
│ │ ├── metrics.rs # Metrics collection
│ │ └── bin/cli.rs # CLI tool
│ └── ebpf/ # eBPF/XDP programs
│ ├── src/xdp.c
│ └── Makefile
├── frontend/ # React dashboard
│ ├── package.json
│ ├── tsconfig.json
│ ├── vite.config.ts
│ ├── index.html
│ └── src/
│ ├── main.tsx # Entry point
│ ├── api.ts # API client
│ ├── components/ # React components
│ ├── hooks/ # Custom hooks
│ └── main.css # Global styles
└── README.md


---

Configuration

Backend

Database: /tmp/netctl.db (configurable in main.rs)

API Port: 3001 (configurable in main.rs)

Logging: RUST_LOG=debug cargo run


Frontend

Backend URL: configured in frontend/vite.config.ts

Port: 5173 (Vite default)

Build output: frontend/dist/



---

Testing

Backend

cd backend
cargo test
cargo test --lib db
cargo test --lib state

Frontend

cd frontend
npm run type-check
npm run lint


---

Performance

Kernel-Level QoS: XDP programs run at network driver level

Zero-Copy Processing: No userspace packet copying overhead

Async Backend: Handles thousands of concurrent connections

Efficient State Storage: Single source of truth in SQLite

Minimal Dependencies: Core functionality with no heavy frameworks



---

Security Notes

Runs with elevated privileges (required for network operations)

SQLite database is local only by default

API is unencrypted (localhost-only in default config)

Extend with JWT authentication (future enhancement)

All network operations are validated before execution



---

Troubleshooting

eBPF Compilation Fails

# Install LLVM tools
sudo apt-get install llvm llvm-dev libelf-dev
# macOS
brew install llvm

XDP Attachment Fails

uname -r # Kernel version must be 5.8+
ethtool -i <interface> | grep -i driver

SQLite Database Locked

pkill netctl-daemon
```

curl <http://localhost:3001/api/health>

## Check proxy config in vite.config.ts

---

Future Enhancements

VPN Integration: WireGuard/OpenVPN support

Deep Packet Inspection: Layer 7 traffic analysis

Firewall Integration: nftables rules

Per-App Traffic Shaping

Bandwidth Analytics: Historical traffic data

Multicast Support: IGMP snooping

Authentication: JWT + RBAC

Audit Logging: Detailed operation history

CLI Expansion: Remote operation support

---

License

MIT License - See LICENSE file

---

Contributing

Ensure code compiles without warnings (cargo check)

Tests pass (cargo test)

Follow existing code style

New features include documentation

Security implications are considered

---

Support

Check existing documentation

Review error messages and logs (RUST_LOG=debug)

Test in isolated environment first

Never run on production networks without testing

---

Architecture Diagram

```bash
Browser (React Dashboard) -> <http://localhost:5173>
             │ HTTP/SSE
             ▼
        Axum API Server (Port 3001)
        ├─ State endpoints
        ├─ VLAN management
        ├─ QoS control
        └─ Metrics streaming
             │
   ┌─────────┼─────────┐
   │ │ │
SQLite DB Transactional State Engine eBPF/XDP Kernel QoS
   │ │ │
   └─────────┼─────────┘
             │
     Network Operations (ip, sysctl, etc)
             │
         Linux Kernel (VLAN, DHCP, QoS)

---
