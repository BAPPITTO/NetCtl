# Releases Overview

NetCtl packages are automatically built and released using GitHub Actions CI/CD pipeline.

## Current Release

**Latest:** v1.0.0  
**Status:** Ready for GitHub Actions build  
**Date:** April 8, 2026

See [v1.0.0.md](v1.0.0.md) for detailed release information, installation instructions, and features.

## Release Workflow

### 🔄 Automated Build Pipeline

1. **Git Tag Creation**
   - Developer creates a version tag: `git tag -a v1.0.0`
   - Tag is pushed to GitHub: `git push origin v1.0.0`
   
2. **GitHub Actions Triggered**
   - Workflow file: `.github/workflows/build-release.yml`
   - Builds packages on multiple Linux distributions
   
3. **Parallel Package Builds**
   
   **Ubuntu 22.04 (Debian/Ubuntu package)**
   - Installs Rust, Node.js, build tools, LLVM
   - Builds Rust backend (`cargo build --release`)
   - Compiles eBPF programs (`cd ebpf && cargo build --release`)
   - Builds React frontend (`npm run build`)
   - Creates `netctl_1.0.0-1_amd64.deb`
   - Validates with `dpkg-deb`
   
   **Rocky Linux 8 (CentOS/RHEL package)**
   - Installs Rust, Node.js, build tools, LLVM
   - Builds Rust backend (`cargo build --release`)
   - Compiles eBPF programs
   - Builds React frontend
   - Creates `netctl-1.0.0-1.el8.x86_64.rpm`
   - Validates with `rpm`

4. **Release Creation**
   - Downloads built packages
   - Generates SHA256 checksums
   - Creates GitHub Release
   - Uploads packages and checksums
   - Publishes release notes

## How to Trigger a Release

### Prerequisites

Ensure you have:
- Git CLI installed
- Push access to repository
- Latest code committed and pushed

### Steps

```bash
cd /Users/jerichofoster/NetCtl

# 1. Create annotated tag
git tag -a v1.0.1 -m "Release 1.0.1 - Bug fixes and improvements"

# 2. Push tag to GitHub
git push origin v1.0.1

# 3. Monitor build progress
# - Go to: https://github.com/jerichofoster/NetCtl/actions
# - Watch the "Build & Release Linux Packages" workflow
# - Builds take 5-15 minutes per package

# 4. Download released packages
# - Go to: https://github.com/jerichofoster/NetCtl/releases
# - Find v1.0.1 release
# - Download .deb and .rpm files
```

## Manual Build (For Testing)

### Requirements

You need a **Linux system** with:
- Rust toolchain (`rustup`)
- Node.js & npm (v18+)
- Build tools: `build-essential`, `llvm-dev`, `clang`
- `cargo` and `npm` in PATH

### Steps

```bash
# Option 1: Using provided script (requires Docker)
bash packaging/docker/build-packages.sh

# Option 2: Native build on Linux
cd /path/to/NetCtl/backend
cargo build --release

cd /path/to/NetCtl/ebpf
cargo build --release

cd /path/to/NetCtl/frontend
npm ci && npm run build

# Then manually create packages (see Dockerfiles for details)
```

## Package Contents

### Debian/Ubuntu (.deb)

```
netctl-1.0.0/
├── DEBIAN/
│   ├── control          # Package metadata
│   ├── postinst         # Post-installation script
│   ├── prerm            # Pre-removal script
│   └── postrm           # Post-removal script
├── usr/bin/
│   ├── netctl           # Main daemon binary
│   └── netctl-tui       # Setup wizard binary
├── var/lib/netctl/
│   └── public/          # React dashboard
├── etc/netctl/          # Configuration directory
├── etc/systemd/system/
│   └── netctl.service   # Systemd service unit
└── var/log/netctl/      # Log directory
```

### CentOS/RHEL (.rpm)

```
netctl-1.0.0-1.el8.x86_64.rpm
  ├── /usr/bin/netctl
  ├── /usr/bin/netctl-tui
  ├── /var/lib/netctl/public/
  ├── /etc/netctl/
  ├── /etc/systemd/system/netctl.service
  └── /var/log/netctl/
```

## Verification

### Download Packages

```bash
# From GitHub Release
cd ~/Downloads
wget https://github.com/jerichofoster/NetCtl/releases/download/v1.0.0/netctl_1.0.0-1_amd64.deb
wget https://github.com/jerichofoster/NetCtl/releases/download/v1.0.0/netctl-1.0.0-1.el8.x86_64.rpm
wget https://github.com/jerichofoster/NetCtl/releases/download/v1.0.0/CHECKSUMS.txt
```

### Verify Checksums

```bash
# Verify all packages at once
sha256sum -c CHECKSUMS.txt

# Or verify individually
sha256sum netctl_1.0.0-1_amd64.deb

# Expected output should match CHECKSUMS.txt
```

### Inspect Package Contents

**Debian:**
```bash
dpkg-deb --info netctl_1.0.0-1_amd64.deb
dpkg-deb --contents netctl_1.0.0-1_amd64.deb
```

**RPM:**
```bash
rpm -qip netctl-1.0.0-1.el8.x86_64.rpm
rpm -qlp netctl-1.0.0-1.el8.x86_64.rpm
```

## Release Management

### Files in This Directory

- **v1.0.0.md** - Release notes and installation guide for v1.0.0
- **README.md** - This file

### Files in `packaging/`

- **release.sh** - Release management script (create tags, instructions)
- **docker/build-packages.sh** - Docker-based build script
- **docker/Dockerfile.debian** - Debian/Ubuntu build image
- **docker/Dockerfile.rpm** - CentOS/RHEL build image

### CI/CD Pipeline

- **.github/workflows/build-release.yml** - GitHub Actions workflow

## Troubleshooting

### Build Failed on GitHub Actions

1. Check workflow logs: https://github.com/jerichofoster/NetCtl/actions
2. Look for specific error in build logs
3. Common issues:
   - Rust compilation error → check source code
   - npm build error → check frontend code
   - Package creation error → check Dockerfile

### Package Installation Fails

**Ubuntu/Debian:**
```bash
# Check dependencies
sudo apt-get install -y \
  libc6 \
  libssl3 \
  dnsmasq

# Try installation again
sudo apt install ./netctl_1.0.0-1_amd64.deb
```

**CentOS/RHEL:**
```bash
# Check dependencies
sudo yum install -y \
  libc \
  openssl \
  dnsmasq

# Try installation again
sudo rpm -i ./netctl-1.0.0-1.el8.x86_64.rpm
```

### Checksums Don't Match

1. Download file again (network corruption possible)
2. Verify file integrity: `ls -la netctl_*.deb netctl_*.rpm`
3. Check CHECKSUMS.txt also downloaded correctly

## Future Releases

### Upcoming (v1.1.0, v2.0.0, etc.)

1. Create feature branch: `git checkout -b feature/new-feature`
2. Implement and test locally
3. Commit and push: `git push origin feature/new-feature`
4. Create Pull Request for review
5. Merge to `main` when approved
6. Create and push new tag: `git tag -a v1.1.0 -m "..."`
7. GitHub Actions automatically builds release

### Breaking Changes

If releasing v2.0.0+ with breaking changes:
1. Update documentation
2. Provide migration guide
3. Note compatibility requirements in release notes
4. Test on multiple distributions

## Version Numbering

NetCtl uses Semantic Versioning:

- **MAJOR.MINOR.PATCH** (e.g., 1.0.0)
- **MAJOR:** Breaking changes
- **MINOR:** New features (backwards compatible)
- **PATCH:** Bug fixes

Examples:
- 1.0.0 → 1.0.1 (patch release)
- 1.0.0 → 1.1.0 (minor release)
- 1.0.0 → 2.0.0 (major release)

---

For questions or issues with releases, see:
- GitHub Issues: https://github.com/jerichofoster/NetCtl/issues
- SECURITY.md: Security vulnerabilities
- README.md: Project overview
