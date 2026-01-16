# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-16

### Added

- Initial release
- S-expression parser for recipe files
- Recipe data structures for all actions:
  - `acquire` - Download sources, binaries, or clone git repos
  - `build` - Extract archives or run build steps
  - `install` - Install files to prefix
  - `configure` - Post-install configuration
  - `start` / `stop` - Service management
  - `remove` - Uninstall packages
- Executor with shell-based command execution
- Variable expansion (`$PREFIX`, `$NPROC`, `$ARCH`, `$BUILD_DIR`)
- Dry-run and verbose modes
- SHA256 checksum verification
- Example recipes for ripgrep, fd, jq, and redis
