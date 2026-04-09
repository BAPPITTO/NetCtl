# NetCtl Security Policy

## Quick Security Steps

Follow these steps before deploying NetCtl to production:

### 1. Network & Access

- Deploy on an isolated network segment or VM  
- Restrict dashboard/API access via firewall rules  
- Expose API only to trusted clients  

### 2. Credentials & Authentication

- Change default admin credentials immediately  
- Use strong, unique passwords  
- Enable JWT authentication if available  
- Avoid storing credentials in version control  

### 3. System & Kernel

- Ensure Linux kernel ≥ 5.8  
- Verify NIC drivers support XDP for eBPF programs  
- Keep system packages and dependencies up to date  
- Monitor for security advisories and apply patches  

### 4. Data & Configuration

- Verify SQLite database has restricted file permissions  
- Backup database regularly  
- Test rollback procedures for network changes  

### 5. Monitoring & Logging

- Enable `RUST_LOG=debug` in testing environments  
- Monitor API logs for suspicious activity  
- Set alerts for failed authentication attempts  
- Track configuration changes  

### 6. Dashboard & API

- Configure HTTPS for production dashboard  
- Ensure API endpoints are not exposed publicly without authentication  
- Verify rate limiting on sensitive endpoints  

### 7. eBPF/XDP

- Confirm eBPF programs run in sandboxed kernel context  
- Validate XDP attachment to network interfaces  
- Test CPU/memory usage under expected traffic load  

> ✅ Following this checklist helps ensure a secure and stable NetCtl deployment.

---

## Reporting a Vulnerability

Please do not report security vulnerabilities through public GitHub issues.  

Instead, report security vulnerabilities to: **<security@netctl.dev>**  

Include the following:

- Description of the vulnerability  
- Steps to reproduce the issue  
- Potential impact  
- Your contact information  
- Any proposed fix (optional)  

We will acknowledge receipt within 48 hours and provide an estimated timeline for a fix.

## Supported Versions

| Version | Supported |
| ---------- | ----------- |
| 1.0.x | ✅ |
| < 1.0 | ❌ |

## Security Features

### Authentication & Authorization

- Dashboard access requires admin credentials  
- Session-based authentication with secure tokens  
- Rate limiting on API endpoints  

### Network Security

- VLAN isolation for network segmentation  
- Idempotent configuration ensures atomic state changes  
- Full rollback capability for failed operations  
- Input validation on all API endpoints  

### Data Protection

- SQLite database with transactional integrity  
- Configuration stored with restricted file permissions  
- Sensitive data (credentials) never logged  
- HTTPS support for dashboard (configurable)  

### Kernel-Level Security (eBPF/XDP)

- eBPF programs run in sandboxed kernel context  
- XDP attachment validates NIC driver support  
- Kernel 5.8+ requirement ensures modern security features  
- CPU and memory limits enforced by kernel  

## Security Best Practices

- Network Isolation: isolated network or VM, firewall rules, trusted clients only  
- Credential Management: change defaults, strong passwords, secure storage  
- System Updates: keep kernel and dependencies updated  
- Monitoring: watch API logs, set alerts for failed authentication, track config changes  
- Backup & Recovery: regularly backup SQLite, test rollbacks, maintain network config backups  

## Known Limitations

- Dashboard requires HTTPS for production  
- eBPF programs require root/CAP_BPF privileges  
- Some network operations may require system reboot  
- XDP not supported on all NIC drivers  

## Security Advisories

- GitHub Security Advisories  
- Project release notes  
- Email notifications  

Subscribe to GitHub releases for timely security updates.

## Compliance

NetCtl is designed with security in mind but makes no guarantees regarding specific compliance standards. Users are responsible for ensuring NetCtl meets their security and compliance requirements.

## Questions?

Contact the security team at **<security@netctl.dev>**.
