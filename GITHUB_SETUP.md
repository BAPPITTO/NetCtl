# NetCtl - GitHub Setup Guide

## Quick Start: Create and Push to GitHub

Follow these steps to create a new GitHub repository and push NetCtl:

### Step 1: Create Repository on GitHub

1. Go to [github.com/new](https://github.com/new)
2. Fill in:
   - **Repository name**: `NetCtl`
   - **Description**: `Enterprise SDN Platform - Network Control Engine with eBPF/XDP, Flow Intelligence, Policy Automation, JWT Auth & Audit Logging`
   - **Visibility**: Public (or Private if preferred)
3. Do NOT initialize with README, .gitignore, or license (we already have these)
4. Click "Create repository"

### Step 2: Add Remote and Push

Copy and run these commands in your terminal:

```bash
cd /Users/jerichofoster/NetCtl

# Add the GitHub remote (replace YOUR_USERNAME with your GitHub username)
git remote add origin https://github.com/YOUR_USERNAME/NetCtl.git

# Rename master branch to main (optional but recommended)
git branch -m master main

# Push the code to GitHub
git push -u origin main
```

### Step 3: Verify on GitHub

After pushing, verify:

- Go to your repository: `https://github.com/YOUR_USERNAME/NetCtl`
- Confirm all files are visible
- Check that commit history shows your initial commit

## Repository Structure

```bash
NetCtl/
├── backend/                          # Rust backend daemon
│   ├── src/
│   │   ├── main.rs                  # Main entry point
│   │   ├── lib.rs                   # Module declarations
│   │   ├── state.rs                 # Transactional state management
│   │   ├── db.rs                    # SQLite persistence
│   │   ├── network/                 # Network operations (VLAN, DHCP, interfaces)
│   │   ├── api/handlers.rs          # REST API endpoints
│   │   ├── api/extensions.rs        # Enterprise API (flows, policies, metrics, audit)
│   │   ├── flow.rs                  # Flow tracking & policy engine (5-tuple, intent automation)
│   │   ├── security.rs              # JWT auth & RBAC (17 permissions, 4 roles)
│   │   ├── timeseries.rs            # Metrics DB & alerting (bounded history, range queries)
│   │   ├── audit.rs                 # Audit logging (compliance, forensics)
│   │   ├── qos.rs                   # QoS management (per-MAC rate limiting)
│   │   ├── metrics.rs               # Device metrics collection & SSE
│   │   └── error.rs                 # Error types
│   ├── ebpf/src/
│   │   ├── xdp.c                    # Basic XDP packet filtering
│   │   └── xdp_shaper.c             # Token bucket traffic shaping (NEW)
│   └── Cargo.toml                   # Rust dependencies
│
├── frontend/                         # TypeScript/React dashboard
│   ├── src/
│   │   ├── main.tsx                 # React entry point
│   │   ├── api.ts                   # REST client wrapper
│   │   ├── components/
│   │   │   ├── Dashboard.tsx        # Main dashboard UI
│   │   │   ├── DeviceList.tsx       # Device management
│   │   │   ├── VLANManager.tsx      # VLAN CRUD
│   │   │   ├── QoSPanel.tsx         # QoS configuration
│   │   │   ├── FlowVisualization.tsx  # Network flow visualization (NEW)
│   │   │   ├── PolicyBuilder.tsx     # Policy creation UI (NEW)
│   │   │   ├── MetricsGraph.tsx      # Time-series charts (NEW)
│   │   │   └── AuditViewer.tsx       # Audit log browser (NEW)
│   │   ├── hooks/
│   │   │   └── useMetricsStream.ts  # SSE metrics hook
│   │   ├── main.css                 # Global styles
│   │   └── main.tsx                 # React app
│   ├── index.html
│   ├── package.json
│   ├── vite.config.ts               # Vite build config
│   └── tsconfig.json
│
├── .github/
│   └── copilot-instructions.md      # Copilot workspace guidelines
│
├── README.md                        # Project overview & quick start
├── BUILD.md                         # Detailed build instructions
├── API.md                           # REST API reference (20+ endpoints)
├── USAGE.md                         # Usage examples & workflows
├── PROJECT_SUMMARY.md               # Implementation summary
├── build.sh                         # Build script
├── start.sh                         # Startup script
└── .gitignore                       # Git exclusions (node_modules, target, .env)
```

## Key Features

### Backend (Rust)

- **Transactional State**: Full rollback support for network operations
- **Flow Intelligence**: 5-tuple tracking, policy matching, intent-based automation
- **JWT Authentication**: Token generation, verification, expiration handling
- **RBAC**: 4 roles (Admin, Manager, User, Guest) with 17 granular permissions
- **Time-Series Metrics**: Bounded history, statistical analysis, threshold-based alerting
- **Audit Logging**: Action tracking with status, actor, resource context
- **eBPF/XDP**: Kernel-space packet filtering and token bucket traffic shaping
- **REST API**: 30+ endpoints covering all features

### Frontend (React + TypeScript)

- **Matrix Cyberpunk UI**: Green-on-black terminal aesthetic
- **Real-Time Flow Visualization**: Top flows by bandwidth with detailed inspection
- **Policy Builder**: Drag-and-drop policy creation with priority management
- **Metrics Dashboard**: Time-series charts, statistics, range queries
- **Audit Viewer**: Log browser with multi-level filtering (actor, action, status)
- **Live Metrics**: SSE streaming for real-time updates

## Build & Run

### Backend

```bash
cd backend
cargo build --release
cargo run
```

### Frontend (Dev)

```bash
cd frontend
npm install
npm run dev
```

### Frontend (Prod)

```bash
cd frontend
npm run build
npm run preview
```

## Testing

```bash
cd backend
cargo test --lib      # Run all unit tests (100+)
cargo test            # Run all tests including integration
```

## Technology Stack

| Component | Technology | Version |
| ----------- | ----------- | --------- |
| Backend Runtime | Tokio | 1.0+ |
| Web Framework | Axum | 0.6+ |
| Database | SQLite | Latest |
| eBPF | libbpf-rs | Latest |
| Frontend Framework | React | 18.0+ |
| Build Tool | Vite | 4.0+ |
| Type System | TypeScript | 5.0+ |

## Development Guidelines (from .github/copilot-instructions.md)

- Place network operations in `backend/src/network/`
- API endpoints in `backend/src/api/`
- Database layer in `backend/src/db/`
- All network changes must be reversible
- Keep API handlers stateless and idempotent
- Use `tokio::` for async operations
- Test eBPF programs before production deployment

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Write tests for new functionality
4. Commit with clear messages: `git commit -m 'Add amazing feature'`
5. Push to your fork: `git push origin feature/amazing-feature`
6. Create a Pull Request

## License

[Add appropriate license - currently none specified]

## Architecture Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                     NetCtl Enterprise SDN Platform          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────┐      ┌─────────────────────────┐ │
│  │   React Dashboard    │      │   REST API (Axum)       │ │
│  │  • Flow Viz          │◄────►│  • 30+ Endpoints        │ │
│  │  • Policies          │      │  • SSE Streaming        │ │
│  │  • Metrics           │      │  • Authentication       │ │
│  │  • Audit Logs        │      │  • Rate Limiting        │ │
│  └──────────────────────┘      └──────────┬──────────────┘ │
│                                           │                 │
│      ┌────────────────────────────────────┼──────────────┐ │
│      │                                    │              │  │
│  ┌───┴──────────────┐  ┌────────────┐  ┌─┴────────────┐ │  │
│  │ Flow Engine      │  │  Security  │  │  Metrics DB  │ │  │
│  │  • 5-Tuple Track │  │  • JWT     │  │  • Alerts    │ │  │
│  │  • Policy Eval   │  │  • RBAC    │  │  • Stats     │ │  │
│  │  • Intent Automn │  │  • Tokens  │  │  • Range QRY │ │  │
│  └──────────────────┘  └────────────┘  └──────────────┘ │  │
│                                                          │  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Transactional State Manager                │  │
│  │  • Device State  • VLANs  • DHCP  • IPv4 Forwarding │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         SQLite Persistent Database                   │  │
│  │  • State Snapshots  • Audit Logs  • Configuration   │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │      Linux Kernel Integration                        │  │
│  │  • XDP Programs (ebpf/)  • Network Commands          │  │
│  │  • Interface Config      • VLAN Operations           │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Project Stats

- **Backend Code**: ~2000 lines (Rust)
- **Frontend Code**: ~1500 lines (TypeScript/React)
- **Test Coverage**: 100+ unit tests
- **API Endpoints**: 30+ routes
- **Supported Permissions**: 17 granular permissions
- **User Roles**: 4 (Admin, Manager, User, Guest)
- **Audit Actions**: 9 types (Create, Read, Update, Delete, Login, Logout, Export, Configure, Execute)
- **Total Files**: 47+

## Next Steps After Publishing

1. **Add CI/CD**: GitHub Actions for automatic testing and building
2. **Docker Support**: Create Dockerfile for containerized deployment
3. **Documentation**: Generate API docs with OpenAPI/Swagger
4. **Plugin System**: Allow third-party eBPF program integration
5. **Multi-Node**: Support distributed control plane
6. **Performance Tuning**: Kernel-space optimization and benchmarking
