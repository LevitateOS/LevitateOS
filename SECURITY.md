# Security policy

## Supported versions

The latest release of each operating system, LevitateOS and AcornOS, is supported. Releases are listed at https://github.com/LevitateOS/releases/releases and described at https://levitateos.org. The first releases (1.0) are previews. Older releases get a fix only by moving to the latest one.

## Reporting a vulnerability

Report it privately through GitHub's private vulnerability reporting for this repository: open the **Security** tab of https://github.com/LevitateOS/LevitateOS and choose **Report a vulnerability**. Please do not open a public issue for a security problem.

Include:

- which release (version, and the ISO or installed system) or which commit is affected;
- what you did, what you expected and what happened;
- the smallest steps or files that reproduce it;
- what an attacker gains from it, if you know.

## What to expect

This is a small project. We read every report and answer it, usually within a few days, and we tell you whether we accept it as a vulnerability. We make no promise about how long a fix takes; we fix the serious ones first and publish the fix as a new release. We credit you in the release notes if you wish.

Problems in software we package but do not write (the Linux kernel, Fedora or Alpine packages, Ginux) are fixed upstream. We pick the fix up in the next release, and we may forward your report.

## Signing keys

Releases are signed with an Ed25519 key. Its public half is committed at `distro/keys/release/2026-11.pub` and is carried by every image under `/usr/share/levitate/release-keys/`. `recupdate` and the installer accept a release only when its `release.json` has a valid signature from a key in that folder. The private key is kept on the owner's machine and is never in CI or in a repository. If a key has to be replaced, the new public key is committed here and announced in the release notes.
