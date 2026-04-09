
Name: netctl
%global git_version %(git describe --tags --abbrev=0 2>/dev/null || echo 1.0.0)
Version: %{git_version}
Release: 1%{?dist}
Summary: Enterprise Network Control Daemon
License: Apache-2.0
URL: https://github.com/BAPPITTO/netctl
Source0: netctl-%{version}.tar.gz

BuildRequires: cargo
BuildRequires: rustc
BuildRequires: openssl-devel
BuildRequires: npm
BuildRequires: python3
BuildRequires: gcc
BuildArch: x86_64 aarch64

Requires: systemd
Requires: libudev
Requires: openssl-libs
Requires: dnsmasq >= 2.75

%description
NetCtl is a production-grade network management daemon with kernel-space
packet shaping capabilities for Linux systems.

Features:
 - Real-time network flow tracking and visualization
 - eBPF/XDP kernel-space traffic shaping
 - Enterprise security with JWT and RBAC
 - Live metrics and audit logging
 - TUI setup wizard
 - Web dashboard with LAN accessibility

Requires Linux kernel 5.8 or later with eBPF/XDP support.

# CLI subpackage
%package cli
Summary: NetCtl Command-Line Interface
Requires: %{name}%{?_isa} = %{version}-%{release}

%description cli
Command-line tools for managing the NetCtl network control daemon.

# Web dashboard subpackage
%package dashboard
Summary: NetCtl Web Dashboard
Requires: %{name} = %{version}-%{release}
Requires: httpd

%description dashboard
Web-based dashboard for monitoring and managing the NetCtl daemon.
Provides real-time network visualization, flow tracking, and configuration.

%prep
%setup -q

%build
# Build backend Rust binaries
cd backend
cargo build --release --lib --bin netctl-daemon --bin netctl-cli --bin netctl-tui
cd ..

# Build frontend dashboard
cd frontend
npm install
npm run build
cd ..

%install
# Create directory structure
install -d %{buildroot}/usr/bin
install -d %{buildroot}/usr/lib/systemd/system
install -d %{buildroot}/etc/netctl/certificates
install -d %{buildroot}/var/lib/netctl
install -d %{buildroot}/var/log/netctl
install -d %{buildroot}/var/run/netctl
install -d %{buildroot}/usr/share/netctl/dashboard
install -d %{buildroot}/usr/share/doc/netctl

# Install backend binaries
install -m 0755 backend/target/release/netctl-daemon %{buildroot}/usr/bin/
install -m 0755 backend/target/release/netctl-cli %{buildroot}/usr/bin/
install -m 0755 backend/target/release/netctl-tui %{buildroot}/usr/bin/

# Install systemd service
install -m 0644 backend/systemd/netctl.service %{buildroot}/usr/lib/systemd/system/

# Install frontend dashboard
cp -r frontend/dist/* %{buildroot}/usr/share/netctl/dashboard/

# Install documentation
install -m 0644 README.md %{buildroot}/usr/share/doc/netctl/
install -m 0644 API.md %{buildroot}/usr/share/doc/netctl/
install -m 0644 USAGE.md %{buildroot}/usr/share/doc/netctl/

# Install example configuration
mkdir -p %{buildroot}/etc/netctl
cat > %{buildroot}/etc/netctl/netctl.env.example << 'EOF'
# NetCtl Environment Configuration
RUST_LOG=info
NETCTL_CONFIG=/etc/netctl/config.toml
EOF

%pre
# Create netctl system user if it doesn't exist
if ! id -u netctl >/dev/null 2>&1; then
    useradd -r -s /sbin/nologin -d /var/lib/netctl -m -G netctl netctl
fi

%post
# Create necessary directories with correct ownership and permissions
mkdir -p /etc/netctl/certificates /var/lib/netctl /var/log/netctl /var/run/netctl

chown -R netctl:netctl /etc/netctl /var/lib/netctl /var/log/netctl /var/run/netctl

chmod 750 /etc/netctl
chmod 700 /etc/netctl/certificates
chmod 750 /var/lib/netctl
chmod 755 /var/log/netctl
chmod 750 /var/run/netctl

# Generate self-signed certificate if missing
if [ ! -f /etc/netctl/certificates/netctl.crt ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout /etc/netctl/certificates/netctl.key \
        -out /etc/netctl/certificates/netctl.crt \
        -subj "/CN=netctl.local/O=NetCtl/C=US" 2>/dev/null || true
    chown netctl:netctl /etc/netctl/certificates/netctl.* 2>/dev/null || true
    chmod 600 /etc/netctl/certificates/netctl.key
    chmod 644 /etc/netctl/certificates/netctl.crt
fi

# Reload systemd
systemctl daemon-reload || true

%preun
# Stop service during uninstall
if [ $1 -eq 0 ]; then
    systemctl stop netctl || true
    systemctl disable netctl || true
fi

%files
/usr/bin/netctl-daemon
/usr/lib/systemd/system/netctl.service
%dir /etc/netctl
/etc/netctl/certificates
%config(noreplace) /etc/netctl/netctl.env.example
/var/lib/netctl
/var/log/netctl
/var/run/netctl
/usr/share/doc/netctl/README.md
/usr/share/doc/netctl/API.md
/usr/share/doc/netctl/USAGE.md

%files cli
/usr/bin/netctl-cli
/usr/bin/netctl-tui

%files dashboard
/usr/share/netctl/dashboard

%changelog
* Mon Apr 08 2024 NetCtl Development <admin@netctl.dev> - 1.0.0-1
- Initial release of NetCtl 1.0.0
- Full eBPF/XDP kernel-space traffic shaping support
- Enterprise security and audit logging
- Web dashboard and TUI setup wizard
- Linux kernel 5.8+ support

%description -l en_US
NetCtl is a comprehensive network control daemon designed for enterprises.
It provides kernel-space packet shaping, real-time flow tracking, access
control, and comprehensive audit logging.