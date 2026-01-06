# Contributing to LevitateOS

Thank you for your interest in contributing to LevitateOS! This project aims to build a modern, safe, and efficient AArch64 kernel in Rust.

## Getting Started

1.  **Read the Documentation**: Familiarize yourself with the project architecture in `docs/ARCHITECTURE.md` and the roadmap in `docs/ROADMAP.md`.
2.  **Follow the SOPs**: We follow strict development practices. Please read the following before starting any work:
    *   `.agent/rules/kernel-development.md`: Rust kernel development SOP.
    *   `.agent/rules/behavior-testing.md`: Testing and traceability SOP.
3.  **Check Open Issues**: Look for issues tagged with "good first issue" or "help wanted".

## Development Workflow

1.  **Fork and Clone**: Create a fork of the repository and clone it locally.
2.  **Create a Team Log**: For any significant work, claim a team number and create a log file in `.teams/TEAM_XXX_<summary>.md`.
3.  **Build and Test**:
    ```bash
    cargo xtask build all
    cargo xtask test
    ```
4.  **Submit a Pull Request**: Ensure your code follows the project's linting rules and all tests pass.

## Code Style

*   We use standard Rust formatting (`cargo fmt`).
*   Lints are strictly enforced. Check `Cargo.toml` for the workspace-wide lint configuration.
*   Avoid `unsafe` code unless absolutely necessary. Document any `unsafe` usage with a `// SAFETY:` comment.

## Community

*   Respect the [Code of Conduct](CODE_OF_CONDUCT.md).
*   Report security vulnerabilities following the [Security Policy](SECURITY.md).
