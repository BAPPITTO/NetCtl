# NetCtl Linux Packaging Guide

This document describes how to build and distribute NetCtl packages for Debian and Red Hat-based Linux distributions.

## Architecture Overview

```bash
packaging/
├── debian/          # Debian/Ubuntu packaging
│   ├── control      # Package metadata
│   ├── rules        # Build recipes
│   ├── postinst     # Post-installation hooks
│   ├── prerm        # Pre-removal hooks
│   └── copyright    # License information
├── rpm/             # Red Hat/CentOS/Fedora packaging
│   └── netctl.spec  # RPM specification
└── systemd/         # Systemd service units
    └── netctl.service

## Building Debian Packages

### Prerequisites

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  cargo \
  rustc \
  debhelper \
  fakeroot \
  dpkg-dev \
  libssl-dev \
  pkg-config \
  npm
```

### Build Commands

```bash
cd /Users/jerichofoster/NetCtl

# Clean previous builds
rm -rf debian/build

# Update debian changelog (optional)
dch -i "New release"

# Build source package
dpkg-source -b . netctl_1.0.0.orig.tar.gz

# Build binary package
dpkg-buildpackage -F -b -uc -us

# List generated packages
ls -lh ../*.deb

# Verify package contents
dpkg -c ../netctl_1.0.0-1_amd64.deb
```

### Generated Debian Packages

1. **netctl_1.0.0-1_amd64.deb** - Main daemon package
   - Binary: `netctl-daemon` → `/usr/bin/`
   - Binary: `netctl-cli` → `/usr/bin/`
   - Binary: `netctl-tui` → `/usr/bin/`
   - Systemd service
   - Configuration in `/etc/netctl/`
   - Logs in `/var/log/netctl/`

2. **netctl-cli_1.0.0-1_amd64.deb** - CLI tools package
   - Depends on: `netctl` main package

3. **netctl-dashboard_1.0.0-1_all.deb** - Web dashboard package
   - Web UI in `/usr/share/netctl/dashboard/`
   - Requires httpd/nginx for serving

### Installation on Debian/Ubuntu

```bash
# Install package
sudo dpkg -i netctl_1.0.0-1_amd64.deb

# Fix dependencies if needed
sudo apt --fix-broken install

# Run setup wizard
sudo netctl-tui

# Start service
sudo systemctl start netctl
sudo systemctl enable netctl

# Check status
sudo systemctl status netctl
```

## Building Red Hat Packages

### Prerequisites for Netctl

```bash
sudo yum install -y \
  cargo \
  rustc \
  openssl-devel \
  rpm-build \
  npm \
  gcc \
  make

# Create build directories
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS,tmp}
```

### Commands to build

```bash
cd /Users/jerichofoster/NetCtl

# Copy spec file
cp packaging/rpm/netctl.spec ~/rpmbuild/SPECS/

# Create source tarball
tar czf ~/rpmbuild/SOURCES/netctl-1.0.0.tar.gz \
  --exclude=.git \
  --exclude=target \
  --exclude=dist \
  --exclude=node_modules \
  .

# Build RPM
rpmbuild -bb ~/rpmbuild/SPECS/netctl.spec

# List generated packages
ls -lh ~/rpmbuild/RPMS/x86_64/

# Verify RPM contents
rpm -qpl ~/rpmbuild/RPMS/x86_64/netctl-1.0.0-1.el8.x86_64.rpm
```

### Generated RPM Packages

1. **netctl-1.0.0-1.el8.x86_64.rpm** - Main daemon package
   - Binary: `netctl-daemon` → `/usr/bin/`
   - Systemd service → `/usr/lib/systemd/system/`
   - Configuration → `/etc/netctl/`
   - Data directory → `/var/lib/netctl/`

2. **netctl-cli-1.0.0-1.el8.x86_64.rpm** - CLI tools
   - Binaries: `netctl-cli`, `netctl-tui`

3. **netctl-dashboard-1.0.0-1.el8.x86_64.rpm** - Web dashboard
   - Dashboard UI → `/usr/share/netctl/dashboard/`

### Installation on Red Hat/CentOS

```bash
# Install package(s)
sudo yum install -y netctl-1.0.0-1.el8.x86_64.rpm

# Run setup wizard
sudo netctl-tui

# Enable and start service
sudo systemctl enable netctl
sudo systemctl start netctl

# Verify installation
sudo systemctl status netctl
sudo journalctl -u netctl -n 50
```

## Package Contents

### Filesystem Layout

```text
/usr/bin/
├── netctl-daemon      # Main daemon executable
├── netctl-cli         # Command-line interface
└── netctl-tui         # Interactive TUI setup

/etc/netctl/
├── config.toml        # Main configuration
├── netctl.env         # Environment variables
└── certificates/      # HTTPS certificates
    ├── netctl.crt
    └── netctl.key

/var/lib/netctl/       # Runtime data
├── state.db           # SQLite state database
├── metrics.db         # Metrics database
└── audit.log          # Audit log file

/var/log/netctl/       # Log files
└── netctl.log

/var/run/netctl/       # Runtime sockets
└── netctl.sock

/usr/share/netctl/dashboard/     # Web dashboard
├── index.html
├── css/
├── js/
└── assets/

/usr/lib/systemd/system/
└── netctl.service     # Systemd service unit
```

## Post-Installation Setup

### 1. Initialize System User

Both Debian and RPM packages automatically create the `netctl` system user:

```bash
# Verify user creation
id netctl
getent passwd netctl
```

### 2. Generate Self-Signed Certificate

Certificate is generated automatically in `/etc/netctl/certificates/`:

- Certificate: `netctl.crt` (365-day validity)
- Private Key: `netctl.key` (2048-bit RSA)

To regenerate:

```bash
sudo netctl-tui  # Use TUI certificate option
```

### 3. Run Setup Wizard

```bash
sudo netctl-tui
# Follow interactive prompts:
# - Network interface selection
# - IP configuration
# - DNS settings
# - Dashboard hostname
# - Admin credentials
```

### 4. Start the Service

```bash
# Enable on boot
sudo systemctl enable netctl

# Start immediately
sudo systemctl start netctl

# Check status
sudo systemctl status netctl

# View logs
sudo journalctl -u netctl -f
```

## Configuration

### Environment File

`/etc/netctl/netctl.env`:

```bash
RUST_LOG=info
NETCTL_CONFIG=/etc/netctl/config.toml
```

### Main Configuration

`/etc/netctl/config.toml`:

```toml
[server]
listen = "0.0.0.0:3001"
host = "netctl.local"
enable_https = true
certificate_path = "/etc/netctl/certificates/netctl.crt"
key_path = "/etc/netctl/certificates/netctl.key"

[database]
path = "/var/lib/netctl/state.db"

[logging]
level = "info"
path = "/var/log/netctl/netctl.log"
```

## Distribution

### Creating Repository

#### Debian Repository

```bash
# Create repository structure
mkdir -p ~/debian-repo/pool/main/n/netctl
cp *.deb ~/debian-repo/pool/main/n/netctl/

# Create package index
cd ~/debian-repo
apt-ftparchive packages pool/main > dists/main/binary-amd64/Packages
gzip -c dists/main/binary-amd64/Packages > dists/main/binary-amd64/Packages.gz
apt-ftparchive release dists/main > dists/main/Release

# Sign repository (optional)
gpg --clearsign -o dists/main/Release.gpg dists/main/Release
```

#### Red Hat Repository

```bash
# Create repository structure
mkdir -p ~/rpm-repo/el/8/x86_64
cp ~/rpmbuild/RPMS/x86_64/*.rpm ~/rpm-repo/el/8/x86_64/

# Create repository metadata
createrepo ~/rpm-repo/el/8/x86_64/

# Sign packages (optional)
rpm --addsign ~/rpm-repo/el/8/x86_64/*.rpm
```

### Hosting

1. **GitHub Releases**: Upload .deb and .rpm files
2. **Package Registry**: Use GitHub Packages, Artifactory, or similar
3. **Auto Repositories**: Set up apt/yum repositories for CI/CD

## Troubleshooting

### Installation Issues

```bash
# Check dependencies
dpkg -I netctl_1.0.0-1_amd64.deb | grep Depends

# Fix broken dependencies
sudo apt --fix-broken install

# View detailed installation log
sudo dpkg -i netctl_1.0.0-1_amd64.deb 2>&1 | less
```

### Service Issues

```bash
# Check service status
sudo systemctl status netctl

# View detailed logs
sudo journalctl -u netctl -n 100
sudo tail -f /var/log/netctl/netctl.log

# Restart service
sudo systemctl restart netctl

# Check if socket is listening
sudo ss -tlnp | grep netctl
```

### Permission Issues

```bash
# Verify file ownership
ls -la /etc/netctl/
ls -la /var/lib/netctl/
ls -la /var/log/netctl/

# Fix ownership
sudo chown -R netctl:netctl /etc/netctl /var/lib/netctl /var/log/netctl

# Fix permissions
sudo chmod 750 /etc/netctl
sudo chmod 700 /etc/netctl/certificates
sudo chmod 750 /var/lib/netctl
```

## Version Management

### Semver Versioning

- **MAJOR**: Breaking API changes, kernel version requirements
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, security patches

Example: `1.2.3` = Major 1, Minor 2, Patch 3

### Release Process

1. Update version in `Cargo.toml` and `packaging/rpm/netctl.spec`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v1.2.3 -m "Release 1.2.3"`
4. Build packages with new version
5. Upload to repositories
6. Create GitHub release with packages

## License

All packages maintain Apache 2.0 license compatibility.
