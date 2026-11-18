# Security policy

This repository holds the release files of LevitateOS and AcornOS: the ISOs, the signed `release.json` manifests, the package parts and `channels/<os>/stable.json`. The project's source and its security reports are in https://github.com/LevitateOS/LevitateOS.

## Supported versions

The latest release of each operating system is supported. The first releases (1.0) are previews.

## Reporting a vulnerability

Report it privately through GitHub's private vulnerability reporting for LevitateOS/LevitateOS: open the **Security** tab of https://github.com/LevitateOS/LevitateOS and choose **Report a vulnerability**. Please do not open a public issue for a security problem, and do not report one here.

Include the release (version and file), what you did, what you expected, what happened, and the smallest steps that reproduce it. If you found a file here that does not match its published hash or signature, say which file and what you compared.

We read every report and answer it, usually within a few days. We make no promise about how long a fix takes; a fix ships as a new release.

## Signing keys

Each release's `release.json` is signed (`release.json.sig`, Ed25519). The public key is `distro/keys/release/2026-11.pub` in LevitateOS/LevitateOS and is carried by every image under `/usr/share/levitate/release-keys/`. Check the signature, and the SHA-256 and size of every file inside the manifest, before you trust a file from this repository. The ISO's SHA-512 is on https://levitateos.org/download/ and in the release notes.
