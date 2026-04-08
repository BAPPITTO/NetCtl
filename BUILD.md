# NetCtl Build & Setup Guide

This guide explains how to build and run NetCtl on your system.

## Prerequisites

### Required
- **Rust 1.70+** ([Install](https://rustup.rs/))
- **Node.js 18+** ([Install](https://nodejs.org/))
- **Linux kernel 5.8+** (for XDP/eBPF support)

### Recommended
- **LLVM tools** (for eBPF compilation)
- **clang** (alternative C compiler for eBPF)
- **Git** (for version control)

## Installation

### 1. Install Rust (if not already installed)

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify:**
```bash
rustc --version
cargo --version
```

### 2. Install Node.js (if not already installed)

**macOS:**
```bash
brew install node
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install nodejs npm
```

**Verify:**
```bash
node --version
npm --version
```

### 3. Install LLVM (for eBPF compilation)

**macOS:**
```bash
brew install llvm
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install llvm llvm-dev libelf-dev
```

## Build

### Automated Full Build (Recommended)

```bash
cd /Users/jerichofoster/NetCtl
bash build.sh
```

This builds:
1. Backend daemon (`netctl-daemon`)
2. CLI tool (`netctl-cli`)
3. **TUI setup wizard** (`netctl-tui`)
4. Frontend dashboard
5. eBPF XDP programs

## Architecture

```
NetCtl Build Targets
├── Backend
│   ├── netctl-daemon      Main network control daemon
│   ├── netctl-cli         Command-line interface tool
│   └── netctl-tui         Interactive setup wizard (NEW)
├── Frontend
│   └── Dashboard          React web UI with LAN config (ENHANCED)
└── Packages
    ├── Debian (.deb)      Multi-package distribution (NEW)
    ├── Red Hat (.rpm)     Enterprise packages (NEW)
    └── Systemd service    Hardened service unit (NEW)
```

### Dependency Graph

```
Backend Dependencies:
├── tokio (async runtime)
├── axum (web framework)
├── serde (serialization)
├── rusqlite (database)
├── libbpf-rs (eBPF integration)
├── ratatui (TUI framework) - NEW
├── crossterm (terminal) - NEW
├── rcgen (certificates) - NEW
└── x509-parser (cert parsing) - NEW

Frontend Dependencies:
├── react
├── typescript
├── vite
├── recharts (graphing)
└── axios (HTTP client)
```

## Troubleshooting

### macOS Issues
If on macOS and compilation fails:
```bash
# macOS doesn't support XDP/eBPF kernel modules
# Skip eBPF compilation with feature flag
cd backend
cargo build --release --no-default-features
```

### Linux TUI Build Issues
```bash
# Ensure terminal is running on Linux with TTY support
uname -s  # Should return 'Linux'
tty      # Should show /dev/pts/X or /dev/ttyX
```

### Package Build Issues
See [PACKAGING.md](PACKAGING.md) for distribution-specific troubleshooting.

The prerequisite checks will:
1. Verify Rust installation
2. Verify Node.js installation
3. Check LLVM availability
4. Build backend binary
5. Install frontend dependencies
6. Build frontend

### Manual Build

**Backend:**
```bash
cd backend
cargo build --release
cd ..
```

Binary location: `backend/target/release/netctl-daemon`

**Frontend:**
```bash
cd frontend
npm install
npm run build
cd ..
```

Output: `frontend/dist/`

## Running

### Production Mode (Recommended)

```bash
# Start backend (requires root for network operations)
sudo ./backend/target/release/netctl-daemon

# In another terminal, serve frontend
cd frontend
python -m http.server 5173 --directory dist
```

Then visit: `http://localhost:5173`

### Development Mode

**Terminal 1 - Backend:**
```bash
cd backend
RUST_LOG=debug cargo run
```

**Terminal 2 - Frontend:**
```bash
cd frontend
npm run dev
```

Dashboard: `http://localhost:5173`
API: `http://localhost:3001`

## Quick Start Script

```bash
bash start.sh
```

This script will:
1. Check if builds exist (build if needed)
2. Start backend daemon with sudo
3. Start frontend dev server
4. Open dashboard in browser

## Testing

### Backend Tests

```bash
cd backend
cargo test
```

### Specific Test Module

```bash
cd backend
cargo test --lib state          # Test state management
cargo test --lib db             # Test database
cargo test --test integration   # Integration tests
```

### Frontend Type Checking

```bash
cd frontend
npm run type-check
```

### Frontend Linting

```bash
cd frontend
npm run lint
```

## Troubleshooting

### Cargo Command Not Found

**Solution:** Source the Rust environment:
```bash
source $HOME/.cargo/env
```

Or add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### eBPF Compilation Fails

**Error:** `error: clang failed with exit status 1`

**Solution:** Install LLVM and development headers:
```bash
# macOS
brew install llvm

# Linux
sudo apt-get install llvm llvm-dev libelf-dev clang
```

### Backend Build Fails on Network Operations

**Error:** `cannot find -lbpf`

**Solution:** Install libbpf development library:
```bash
# macOS
brew install libbpf

# Linux
sudo apt-get install libbpf-dev
```

### Permission Denied When Running Backend

**Error:** `Permission denied (os error 13)`

**Solution:** Run with sudo:
```bash
sudo ./backend/target/release/netctl-daemon
```

### Frontend Can't Connect to Backend

**Error:** `Failed to fetch /api/state`

**Solution:**
1. Verify backend is running: `curl http://localhost:3001/api/health`
2. Check CORS settings in backend
3. Verify API proxy in `frontend/vite.config.ts`

## Environment Variables

### Backend
```bash
RUST_LOG=debug          # Enable debug logging
NETCTL_DB=/path/to.db  # Custom database path
NETCTL_PORT=3001       # API port
```

### Frontend
```bash
VITE_API_URL=http://localhost:3001  # Backend API URL
```

## Project Structure After Build

```
netctl/
├── backend/
│   ├── target/release/
│   │   ├── netctl-daemon
│   │   └── netctl-cli
│   └── ...
├── frontend/
│   ├── dist/               # Built frontend
│   │   ├── index.html
│   │   ├── assets/
│   │   └── ...
│   └── ...
└── build.sh, start.sh
```

## Performance Optimization

### Backend Release Build

The default `cargo build --release` includes optimizations:
- LTO (Link-Time Optimization)
- Single codegen unit
- Level 3 optimization

For maximum performance:
```bash
cd backend
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Frontend Build

Production build is already optimized:
```bash
npm run build  # Minified, tree-shaken, optimized
```

## Getting Help

If you encounter issues:

1. **Check logs:**
   ```bash
   # Backend logs
   RUST_LOG=debug cargo run
   
   # Frontend logs (browser console)
   F12 → Console tab
   ```

2. **Verify compilation:**
   ```bash
   cd backend && cargo check
   cd ../frontend && npm run type-check
   ```

3. **Test API manually:**
   ```bash
   curl http://localhost:3001/api/health
   ```

## Platform-Specific Notes

### macOS
- XDP support requires running on Linux VM (XDP not available on macOS)
- Use remote Linux server or Docker container for testing eBPF programs
- Frontend development works normally on macOS

### Linux
- Full XDP/eBPF support
- Ensure kernel is 5.8+: `uname -r`
- Network operations require root/CAP_NET_ADMIN

## Next Steps

1. ✅ Build complete - proceed to [API Documentation](./API.md)
2. ✅ Run dashboard - see [Usage Guide](./USAGE.md)
3. ✅ Deploy to server - see [Deployment Guide](./DEPLOYMENT.md)
