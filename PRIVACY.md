# LevitateOS Privacy Policy

**Effective Date:** 2026-01-22
**Last Updated:** 2026-01-22

## Summary

LevitateOS collects **zero** data from users. There is no telemetry, no analytics, no phone-home behavior.

## Data Collection

### What We Collect

Nothing. LevitateOS does not:

- Send telemetry data
- Phone home to any server
- Track usage patterns
- Collect crash reports automatically
- Harvest system information
- Contact external servers without explicit user action

### Network Connections

LevitateOS only makes network connections when **you** initiate them:

| Action | Connection | Your Choice |
|--------|------------|-------------|
| `recipe update` | Package repository | You run the command |
| `recipe install` | Package downloads | You run the command |
| Web browsing | Websites you visit | You open them |
| System updates | Update server | You enable updates |

The base system makes **zero** unsolicited network connections.

## GDPR Compliance

LevitateOS is designed with GDPR principles:

- **Data Minimization**: We collect nothing, the minimum possible
- **Privacy by Default**: No opt-out needed because there's nothing to opt out of
- **User Control**: All your data stays on your machine
- **Transparency**: This document tells you everything (which is nothing)

## Third-Party Software

Software you install via `recipe install` may have its own privacy policies. We recommend reviewing the privacy practices of any software you install. LevitateOS itself does not add any tracking to third-party packages.

## Crash Reporting

There is no automatic crash reporting. If you experience issues:

1. You choose whether to report them
2. You choose what information to include
3. Reports go to our public issue tracker (GitHub)
4. You control the entire process

## Future Changes

If we ever add any data collection (we don't plan to):

1. It will be **opt-in only** (disabled by default)
2. We will clearly document what is collected
3. We will explain why it's needed
4. You will have full control to disable it
5. This document will be updated

## Verification

You can verify our privacy claims:

```bash
# Check for running network services
ss -tulpn

# Monitor network connections
tcpdump -i any

# Review systemd services
systemctl list-units --type=service

# Check for cron jobs or timers
systemctl list-timers
```

The source code is open. Audit it yourself.

## Contact

For privacy questions: Open an issue at https://github.com/LevitateOS/LevitateOS

---

**TL;DR**: We don't collect your data. Period.
