# Security Policy

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities to: **[security@netctl.dev](mailto:security@netctl.dev)**

Please include the following information:
- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Your contact information
- Any proposed fix (optional)

We will acknowledge receipt of your report within 48 hours and provide an estimated timeline for a fix.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Security Features

NetCtl implements several security measures to protect network operations:

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

When deploying NetCtl:

1. **Network Isolation**
   - Run on isolated network segment or VM
   - Use firewall rules to restrict dashboard access
   - Only expose API to trusted clients

2. **Credential Management**
   - Change default admin credentials immediately
   - Use strong, unique passwords
   - Store credentials securely (avoid version control)

3. **System Updates**
   - Keep Linux kernel updated (5.8+)
   - Monitor Dependabot alerts for dependency updates
   - Apply security patches promptly

4. **Monitoring**
   - Monitor API logs for suspicious activity
   - Set up alerts for failed authentication attempts
   - Track configuration changes

5. **Backup & Recovery**
   - Regularly backup SQLite database
   - Test rollback procedures periodically
   - Maintain backup of network configuration

## Known Limitations

- Dashboard requires HTTPS configuration for production deployments
- eBPF programs require root/CAP_BPF privileges
- Some network operations may require system reboot
- XDP not supported on all NIC drivers

## Security Advisories

Security advisories will be published via:
- GitHub Security Advisories
- Project release notes
- Email notifications

Subscribe to GitHub releases for timely security updates.

## Compliance

NetCtl is designed with security in mind but make no guarantees regarding specific compliance standards. Users are responsible for ensuring NetCtl meets their security and compliance requirements.

## Questions?

For security-related questions or concerns, contact the security team at **security@netctl.dev**.
