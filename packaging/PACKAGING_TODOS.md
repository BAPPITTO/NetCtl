# 1. Pre-Build Setup (One-time)

*Install build tools and dependencies**

- **Ubuntu/Debian**

```bash
sudo apt-get install build-essential devscripts debhelper cargo rustc llvm-dev clang

CentOS/RHEL


sudo yum install make gcc rpm-build cargo rust llvm-devel clang

Clone or update the NetCtl repository


git clone https://github.com/BAPPITTO/NetCtl.git
cd NetCtl && git pull origin main


---

2. Backend Build (Rust)

Build backend in release mode


cd backend && cargo build --release

Verify binaries exist


ls -la target/release/netctl

Run backend tests


cargo test --release

Verify eBPF programs compile


ls -la ebpf/src/
cd ebpf && cargo build --release

Create binary distribution


mkdir -p /tmp/netctl-dist/usr/bin
cp backend/target/release/netctl /tmp/netctl-dist/usr/bin/netctl
cp backend/target/release/netctl-tui /tmp/netctl-dist/usr/bin/netctl-tui
chmod 755 /tmp/netctl-dist/usr/bin/*


---

3. Frontend Build (TypeScript/React)

Build frontend


cd frontend
npm ci
npm run build

Verify output


ls -la dist/

Create web assets distribution


mkdir -p /tmp/netctl-dist/var/lib/netctl/public
cp -r frontend/dist/* /tmp/netctl-dist/var/lib/netctl/public/
chmod 644 /tmp/netctl-dist/var/lib/netctl/public/*


---

4. Configuration and Systemd Setup

Create directories


mkdir -p /tmp/netctl-dist/etc/netctl
mkdir -p /tmp/netctl-dist/etc/netctl/ssl
mkdir -p /tmp/netctl-dist/var/lib/netctl
mkdir -p /tmp/netctl-dist/var/log/netctl

Copy configuration templates


cp backend/config.example.toml /tmp/netctl-dist/etc/netctl/config.toml.example
chmod 600 /tmp/netctl-dist/etc/netctl/config.toml.example

Copy systemd service file


mkdir -p /tmp/netctl-dist/etc/systemd/system
cp packaging/systemd/netctl.service /tmp/netctl-dist/etc/systemd/system/
systemd-analyze verify /tmp/netctl-dist/etc/systemd/system/netctl.service


---

5. Debian/Ubuntu Package Build

Verify Debian structure


ls -la packaging/debian/netctl_1.0.0-1_amd64/DEBIAN/

Check control, postinst, prerm, postrm

Update version numbers if needed


grep Version packaging/debian/netctl_1.0.0-1_amd64/DEBIAN/control

Build package


cd packaging/debian
dpkg-deb --build netctl_1.0.0-1_amd64

Verify and validate


dpkg-deb --info netctl_1.0.0-1_amd64.deb
dpkg-deb --contents netctl_1.0.0-1_amd64.deb | head -20
lintian netctl_1.0.0-1_amd64.deb

Test package


sudo apt-get install ./netctl_1.0.0-1_amd64.deb
which netctl && which netctl-tui
sudo systemctl start netctl
sudo systemctl status netctl
sudo apt-get remove netctl


---

6. CentOS/RHEL Package Build

Verify RPM structure


ls -la packaging/rpm/netctl-1.0.0-1.el8.x86_64/

Check SPECS/netctl.spec and version


grep "^Version:" SPECS/netctl.spec

Build RPM


cd packaging/rpm
rpmbuild -ba SPECS/netctl.spec

Verify and validate


rpm -qip RPMS/x86_64/netctl-1.0.0-1.el8.x86_64.rpm
rpm -qlp RPMS/x86_64/netctl-1.0.0-1.el8.x86_64.rpm | head -20

Test RPM


sudo yum install ./netctl-1.0.0-1.el8.x86_64.rpm
which netctl && which netctl-tui
sudo systemctl start netctl
sudo systemctl status netctl
sudo yum remove netctl


---

7. Release and Distribution

Create GitHub release


git tag -a v1.0.0 -m "Release 1.0.0"
git push origin v1.0.0

Upload packages to release:

netctl_1.0.0-1_amd64.deb

netctl-1.0.0-1.el8.x86_64.rpm


Add SHA256 checksums


sha256sum netctl_1.0.0-1_amd64.deb
sha256sum netctl-1.0.0-1.el8.x86_64.rpm

Update documentation: USAGE.md, README.md, installation instructions



---

8. Post-Release Verification

Test fresh installations:

Ubuntu 22.04 LTS

CentOS 8 / AlmaLinux 8


Verify all features: dashboard, VLAN, QoS rules

Monitor package manager integration and dependencies

Update CI/CD pipeline:

Automated builds via GitHub Actions

Automated testing in CI




---

9. Troubleshooting Checklist

Binary compatibility: GLIBC, eBPF kernel modules

Missing dependencies: ldd netctl

Systemd service issues: logs, permissions

Permission errors: verify capabilities:


getcap /usr/bin/netctl

Ensure CAP_SYS_ADMIN and CAP_BPF are granted to service



---

Notes

Packages built for x86_64 by default; ARM64 optional

Kernel requirement: 5.8+

SQLite DB initialized automatically on first run

Consider GPG signing packages for production releases
