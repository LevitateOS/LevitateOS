# LevitateOS Security Policy

**Effective Date:** 2026-01-22
**Last Updated:** 2026-01-22

## Supported Versions

| Version | Supported | Notes |
|---------|-----------|-------|
| Latest release | Yes | Full security support |
| Previous release | 6 months | Critical fixes only |
| Older releases | No | Please upgrade |

## Reporting a Vulnerability

### For Security Issues

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report security issues via:

1. **Email:** security@levitateos.org (preferred)
2. **GitHub Security Advisories:** Use the "Report a vulnerability" button on our repository

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Any suggested fixes (optional)

### Response Timeline

| Stage | Timeframe |
|-------|-----------|
| Initial acknowledgment | 48 hours |
| Severity assessment | 7 days |
| Fix development | Depends on severity |
| Public disclosure | After fix is released |

### Severity Levels

| Level | Response Time | Examples |
|-------|---------------|----------|
| Critical | 24-48 hours | Remote code execution, privilege escalation |
| High | 7 days | Local privilege escalation, data exposure |
| Medium | 30 days | Denial of service, information disclosure |
| Low | Next release | Minor issues, hardening improvements |

## Security Update Policy

### How Updates Are Delivered

1. **Package updates** via `recipe upgrade`
2. **Kernel updates** via `recipe upgrade linux`
3. **Security advisories** published on website and GitHub

### Update Frequency

- **Critical vulnerabilities:** Emergency release within 48 hours
- **High severity:** Patch release within 7 days
- **Regular security updates:** Included in monthly releases
- **Kernel updates:** Track stable kernel releases (usually weekly)

### User Responsibility

LevitateOS does not auto-update. Users should:

```bash
# Check for updates regularly
recipe update
recipe upgrade

# Subscribe to security announcements
# (via GitHub releases or mailing list)
```

## Security Design Principles

### Defense in Depth

1. **Minimal base system** - Fewer packages = smaller attack surface
2. **No telemetry** - No data exfiltration vectors
3. **No mandatory services** - You enable what you need
4. **Principle of least privilege** - Services run as unprivileged users where possible

### What We Don't Do

- No remote management enabled by default
- No listening network services in base install
- No automatic connections to external servers
- No closed-source components in base system

## Known Security Considerations

### Current Limitations

- SELinux/AppArmor not enabled by default (planned for future)
- Secure Boot not yet supported (planned for future)
- No automatic security updates (by design - user controls updates)

### Hardening Recommendations

For security-sensitive deployments:

```bash
# Enable firewall
recipe install nftables
systemctl enable nftables

# Disable root SSH login (if SSH installed)
# Edit /etc/ssh/sshd_config: PermitRootLogin no

# Use strong passwords or key-based auth
# Regular updates
recipe update && recipe upgrade
```

## Upstream Security

LevitateOS inherits security from:

| Component | Source | Security Contact |
|-----------|--------|------------------|
| Kernel | kernel.org | security@kernel.org |
| Userspace packages | Rocky Linux / Fedora | Rocky Security Team |
| systemd | systemd project | systemd security |

We monitor upstream security advisories and incorporate fixes promptly.

## Audit and Verification

LevitateOS is fully open source. You can:

1. **Audit the source code** - All code is public
2. **Verify builds** - Build system is documented
3. **Check package signatures** - Packages are signed
4. **Review configurations** - All configs are visible

## CRA Compliance (EU Cyber Resilience Act)

In preparation for CRA requirements (effective December 2027):

- [ ] Vulnerability disclosure process (this document)
- [ ] Security update policy (this document)
- [ ] SBOM generation (planned)
- [ ] Reproducible builds (planned)
- [ ] No known exploitable vulnerabilities at release (ongoing)

## Contact

- **Security issues:** security@levitateos.org
- **General questions:** Open a GitHub issue
- **Website:** https://levitateos.org

---

**Remember:** Security is a shared responsibility. Keep your system updated.
