<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## NetCtl - Network Control Engine

### Project Overview

NetCtl is a production-grade network management daemon in Rust with a TypeScript/React dashboard.

**Tech Stack:**
- Backend: Rust (Tokio async, eBPF/XDP, SQLite)
- Frontend: TypeScript/React (Vite, Matrix-style dashboard)
- Target: Linux kernel 5.8+

### Architecture

- Transactional state management with full rollback
- Idempotent VLAN/DHCP configuration
- eBPF/XDP kernel-level QoS
- Live metrics via SSE streaming
- RESTful API layer
- Matrix cyberpunk UI

### Development Guidelines

**Backend Development:**
- Place network operations in `/backend/src/network/`
- API endpoints in `/backend/src/api/`
- Database layer in `/backend/src/db/`
- State transactions in `/backend/src/state.rs`
- eBPF programs in `/backend/ebpf/src/` (C language)

**Frontend Development:**
- React components in `/frontend/src/components/`
- Custom hooks in `/frontend/src/hooks/`
- API client wrapper in `/frontend/src/api.ts`
- Vite configuration for dev/prod builds

**Code Standards:**
- Use `tokio::` for async runtime
- Transactional semantic for all network state changes
- Test eBPF programs before production deployment
- Keep API handlers stateless and idempotent

### Build & Run

**Backend:**
```bash
cd backend
cargo build --release
cargo run
```

**Frontend:**
```bash
cd frontend
npm install
npm run dev      # Development
npm run build    # Production
```

### Testing

**Unit Tests:**
```bash
cd backend
cargo test
```

**Integration Tests:**
- Verify VLAN creation via API
- Test DHCP scope generation
- Validate transaction rollback
- Confirm XDP attachment on NIC

**System Integration:**
- Test on Linux VM with kernel 5.8+
- Isolated network environment
- Verify no system connectivity loss

### Important Notes

- All network changes must be reversible (rollback support)
- VLAN operations should use 802.1Q standard
- DHCP uses dnsmasq backend
- QoS uses eBPF XDP for kernel-space packet filtering
- Database: SQLite with embedded transactions
- API: RESTful with SSE for metrics streaming

### Common Tasks

**Add new network operation:**
1. Define `NetOp` variant in `state.rs`
2. Implement reverse operation for rollback
3. Add system command wrapper in `network/` module
4. Create API endpoint in `api/` module
5. Add unit test for transaction

**Update dashboard:**
1. Modify React component in `/frontend/src/components/`
2. Update API client in `/frontend/src/api.ts`
3. Test with backend running on `localhost:3001`

**Extend metrics:**
1. Update BPF map reading in `metrics.rs`
2. Add SSE event type
3. Create React hook in `/frontend/src/hooks/useMetricsStream.ts`

### Troubleshooting

- **eBPF compilation fails:** Check LLVM version (needs `llvm-tools`)
- **XDP attachment fails:** Ensure kernel supports XDP (5.8+), check NIC driver
- **SQLite locks:** Verify no multiple daemon instances running
- **Frontend dev proxy:** Check frontend `vite.config.ts` backend URL is `http://localhost:3001`
