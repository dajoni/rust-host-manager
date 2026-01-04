# Project Context

## Purpose
Provide a Rust service that runs on a Linux host to manage installed `.deb` packages, including endpoints for uploading archives containing metadata and `.deb` files to verify the archive targets the intended host.

## Tech Stack
- Rust
- HTTP server framework with TLS-PSK support (to be selected during implementation)

## Project Conventions

### Code Style
- Keep implementations small and direct.
- Prefer clear, explicit configuration via environment variables.

### Architecture Patterns
- Separate binaries for service and CLI.
- Thin handlers with testable core logic.

### Testing Strategy
- Unit tests for handlers and CLI output.
- Run `cargo test` for validation.

### Git Workflow
- Use `main` as the default branch.
- Create feature branches for work (`feature/<short-name>` or `fix/<short-name>`).
- Merge to `main` via merge requests.
- Releases are cut by tagging `main` after merge (annotated tags preferred).

## Domain Context
- The service manages `.deb` packages on a Linux host.
- The service accepts archive uploads that include metadata for host verification alongside `.deb` files.
- The service supports a plugin architecture where archives can install `.deb` packages containing plugins that extend server functionality.

## Important Constraints
- TLS-PSK support is required for the service when enabled by environment variables.
- Plugin functionality is delivered via installable `.deb` packages.

## External Dependencies
- None specified.
