# Linux Package Prebuilding Todos

Comprehensive checklist for building and releasing NetCtl Linux packages for Debian/Ubuntu and CentOS/RHEL.

## Pre-Build Setup (One-time)

- [ ] Install build tools and dependencies
  - [ ] Ubuntu/Debian: `sudo apt-get install build-essential devscripts debhelper cargo rustc`
  - [ ] CentOS/RHEL: `sudo yum install make gcc rpm-build cargo rust`
- [ ] Install LLVM tools for eBPF compilation
  - [ ] Ubuntu/Debian: `sudo apt-get install llvm-dev clang`
  - [ ] CentOS/RHEL: `sudo yum install llvm-devel clang`
- [ ] Clone/update NetCtl repository
  - [ ] `git clone https://github.com/jerichofoster/NetCtl.git`
  - [ ] `cd NetCtl && git pull origin main`

## Backend Build (Rust)

- [ ] Build Rust backend in release mode
  - [ ] `cd backend && cargo build --release`
  - [ ] Verify binary exists: `ls -la target/release/netctl`
  - [ ] Run backend tests: `cargo test --release`
- [ ] Verify eBPF programs compile
  - [ ] Check eBPF source: `ls -la ebpf/src/`
  - [ ] Compile eBPF: `cd ebpf && cargo build --release`
- [ ] Create binary distribution
  - [ ] `mkdir -p /tmp/netctl-dist/usr/bin`
  - [ ] `cp backend/target/release/netctl /tmp/netctl-dist/usr/bin/netctl`
  - [ ] `cp backend/target/release/netctl-tui /tmp/netctl-dist/usr/bin/netctl-tui`
  - [ ] Set permissions: `chmod 755 /tmp/netctl-dist/usr/bin/*`

## Frontend Build (TypeScript/React)

- [ ] Build React frontend
  - [ ] `cd frontend && npm ci` (clean install)
  - [ ] `npm run build`
  - [ ] Verify build output: `ls -la dist/`
- [ ] Create web assets distribution
  - [ ] `mkdir -p /tmp/netctl-dist/var/lib/netctl/public`
  - [ ] `cp -r frontend/dist/* /tmp/netctl-dist/var/lib/netctl/public/`
  - [ ] Set permissions: `chmod 644 /tmp/netctl-dist/var/lib/netctl/public/*`

## Configuration and Systemd Setup

- [ ] Create configuration directory structure
  - [ ] `mkdir -p /tmp/netctl-dist/etc/netctl`
  - [ ] `mkdir -p /tmp/netctl-dist/etc/netctl/ssl`
  - [ ] `mkdir -p /tmp/netctl-dist/var/lib/netctl`
  - [ ] `mkdir -p /tmp/netctl-dist/var/log/netctl`
- [ ] Copy configuration templates
  - [ ] `cp backend/config.example.toml /tmp/netctl-dist/etc/netctl/config.toml.example`
  - [ ] Set permissions: `chmod 600 /tmp/netctl-dist/etc/netctl/config.toml.example`
- [ ] Copy systemd service file
  - [ ] `mkdir -p /tmp/netctl-dist/etc/systemd/system`
  - [ ] `cp packaging/systemd/netctl.service /tmp/netctl-dist/etc/systemd/system/`
  - [ ] Verify service file syntax: `systemd-analyze verify /tmp/netctl-dist/etc/systemd/system/netctl.service`

## Debian/Ubuntu Package Build

- [ ] Verify Debian directory structure
  - [ ] `ls -la packaging/debian/netctl_1.0.0-1_amd64/DEBIAN/`
  - [ ] Check files: `control`, `postinst`, `prerm`, `postrm`
- [ ] Update version numbers if needed
  - [ ] Edit `packaging/debian/netctl_1.0.0-1_amd64/DEBIAN/control`
  - [ ] Check version matches: `grep Version control`
- [ ] Build Debian package
  - [ ] `cd packaging/debian && dpkg-deb --build netctl_1.0.0-1_amd64`
  - [ ] Verify: `ls -la netctl_1.0.0-1_amd64.deb`
- [ ] Validate Debian package
  - [ ] `dpkg-deb --info netctl_1.0.0-1_amd64.deb`
  - [ ] `dpkg-deb --contents netctl_1.0.0-1_amd64.deb | head -20`
  - [ ] Lintian check: `lintian netctl_1.0.0-1_amd64.deb`
- [ ] Test Debian package (in VM or container)
  - [ ] `sudo apt-get install ./netctl_1.0.0-1_amd64.deb`
  - [ ] Verify installation: `which netctl && which netctl-tui`
  - [ ] Verify service: `sudo systemctl start netctl && sudo systemctl status netctl`
  - [ ] Cleanup: `sudo apt-get remove netctl`

## CentOS/RHEL Package Build

- [ ] Verify RPM directory structure
  - [ ] `ls -la packaging/rpm/netctl-1.0.0-1.el8.x86_64/`
  - [ ] Check `SPECS/netctl.spec` exists
- [ ] Update version numbers if needed
  - [ ] Edit `SPECS/netctl.spec`
  - [ ] Check version: `grep "^Version:" SPECS/netctl.spec`
- [ ] Build RPM package
  - [ ] `cd packaging/rpm`
  - [ ] `rpmbuild -ba SPECS/netctl.spec`
  - [ ] Verify: `ls -la RPMS/x86_64/netctl-1.0.0-1.el8.x86_64.rpm`
- [ ] Validate RPM package
  - [ ] `rpm -qip netctl-1.0.0-1.el8.x86_64.rpm`
  - [ ] `rpm -qlp netctl-1.0.0-1.el8.x86_64.rpm | head -20`
- [ ] Test RPM package (in VM or container)
  - [ ] `sudo yum install ./netctl-1.0.0-1.el8.x86_64.rpm`
  - [ ] Verify installation: `which netctl && which netctl-tui`
  - [ ] Verify service: `sudo systemctl start netctl && sudo systemctl status netctl`
  - [ ] Cleanup: `sudo yum remove netctl`

## Release and Distribution

- [ ] Create GitHub release
  - [ ] Tag version: `git tag -a v1.0.0 -m "Release 1.0.0"`
  - [ ] Push tag: `git push origin v1.0.0`
  - [ ] Create GitHub release from tag
- [ ] Upload packages to release
  - [ ] Attach `netctl_1.0.0-1_amd64.deb`
  - [ ] Attach `netctl-1.0.0-1.el8.x86_64.rpm`
  - [ ] Add SHA256 checksums to release notes
    ```bash
    sha256sum netctl_1.0.0-1_amd64.deb
    sha256sum netctl-1.0.0-1.el8.x86_64.rpm
    ```
- [ ] Update documentation
  - [ ] Update version in `USAGE.md`
  - [ ] Update `README.md` with download links
  - [ ] Update installation instructions
- [ ] Verify download links
  - [ ] Test package download from release page
  - [ ] Verify SHA256 checksums match

## Post-Release Verification

- [ ] Test fresh installations on clean systems
  - [ ] Ubuntu 22.04 LTS
  - [ ] CentOS 8 / AlmaLinux 8
  - [ ] Verify all features work: dashboard access, VLAN creation, QoS rules
- [ ] Monitor for package manager integration
  - [ ] Check if packages appear in distro repos (if submitted)
  - [ ] Verify dependency resolution
- [ ] Update CI/CD pipeline
  - [ ] Enable automated package building in GitHub Actions
  - [ ] Set up automated testing in CI matrix

## Troubleshooting Checklist

- [ ] Binary compatibility issues
  - [ ] Check GLIBC version compatibility
  - [ ] Verify eBPF kernel module loads
- [ ] Missing dependencies
  - [ ] Review output of `ldd netctl` for unmet dependencies
  - [ ] Add to package dependencies if needed
- [ ] Systemd service issues
  - [ ] Check service logs: `sudo journalctl -u netctl -n 50`
  - [ ] Verify file permissions on config/data directories
- [ ] Permission errors
  - [ ] Ensure capabilities set: `getcap /usr/bin/netctl`
  - [ ] Verify CAP_SYS_ADMIN or CAP_BPF granted to service

## Notes

- All packages built for x86_64 architecture; add ARM64 builds if needed
- Kernel requirement: 5.8+ (documented in package control/spec files)
- Database (SQLite) initialized on first run; no manual setup needed
- Consider GPG signing packages for production releases
