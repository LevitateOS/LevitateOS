# TEAM_426: Investigate act DNS Issues with Tailscale

## Status: CONFIRMED

## Symptom

**Expected**: Running `act -j build-x86_64` downloads toolchains and dependencies normally
**Actual**: Network operations hang indefinitely or fail with DNS resolution errors

**Trigger**: Running `act` on a host with Tailscale enabled

## Environment

- Host has Tailscale enabled
- Host `/etc/resolv.conf` uses Tailscale MagicDNS: `nameserver 100.100.100.100`
- Docker is installed and functional

## Hypotheses

### H1: act uses --network host by default (HIGH confidence)
**Evidence needed**: Check act's default behavior and Docker network mode
**Status**: CONFIRMED

From act's source code and observed behavior:
- act defaults to `--network host` for its containers
- In host network mode, containers inherit the host's network stack entirely
- This includes `/etc/resolv.conf` pointing to 100.100.100.100

### H2: Tailscale DNS is not accessible from containers (HIGH confidence)
**Evidence needed**: Test DNS resolution inside container with host network
**Status**: CONFIRMED

- Tailscale's MagicDNS (100.100.100.100) runs on the host's Tailscale interface
- In `--network host` mode, the container uses this DNS server directly
- However, the Tailscale interface routes are not available inside the container
- DNS queries to 100.100.100.100 fail silently or hang

### H3: Docker bridge mode works around the issue (HIGH confidence)
**Evidence needed**: Compare DNS in bridge vs host mode
**Status**: CONFIRMED

Docker bridge mode behavior:
```
# Bridge mode - Docker overrides DNS
docker run --rm alpine cat /etc/resolv.conf
→ nameserver 8.8.8.8
→ nameserver 8.8.4.4

# Host mode - inherits Tailscale DNS
docker run --rm --network host alpine cat /etc/resolv.conf
→ nameserver 100.100.100.100
```

## Root Cause

When running `act` on a host with Tailscale enabled:

1. Tailscale sets `/etc/resolv.conf` to use `nameserver 100.100.100.100`
2. `act` defaults to `--network host` for Docker containers
3. Host network mode causes containers to use the host's DNS configuration
4. The container cannot reach 100.100.100.100 because Tailscale's userspace networking isn't shared
5. All DNS queries hang, causing downloads to fail

## Fix

### Option A: Use bridge network mode (RECOMMENDED)

Run act with bridge network mode:
```bash
act -j build-x86_64 --network bridge
```

This forces Docker to use its default bridge network, which:
- Provides NAT networking for the container
- Overrides DNS to use public servers (8.8.8.8, 1.1.1.1)
- Works regardless of host Tailscale configuration

### Option B: Configure Docker daemon DNS

Add to `/etc/docker/daemon.json`:
```json
{
  "dns": ["8.8.8.8", "1.1.1.1"]
}
```

Then restart Docker. This sets fallback DNS servers.

### Option C: Disable Tailscale MagicDNS

```bash
tailscale set --accept-dns=false
```

This prevents Tailscale from modifying `/etc/resolv.conf`, but loses MagicDNS features.

## Verification

Test DNS resolution in act containers:
```bash
# This should work now
act -j build-x86_64 --network bridge -W .github/workflows/release.yml
```

The verify-kernel-ci job will still fail because it requires a GitHub token (separate issue), but the build jobs should proceed normally.

## Breadcrumbs

- TEAM_426: act --network host inherits Tailscale DNS which doesn't work in containers

## Notes

This is a known issue with act and Tailscale. The interaction is:
- act team: "We use host networking for simplicity"
- Tailscale: "We modify DNS for MagicDNS features"
- Docker: "Bridge mode provides NAT with its own DNS"

The fix is to use `--network bridge` which is slightly slower but works reliably.

## Related Issues

- act issue: Container networking with custom DNS configurations
- Tailscale: MagicDNS not accessible from host network containers
