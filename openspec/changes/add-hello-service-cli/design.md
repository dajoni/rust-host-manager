## Context
We are introducing two minimal Rust binaries: a REST service and a CLI. The service must be capable of TLS with pre-shared keys (PSK) without introducing unnecessary complexity.

## Goals / Non-Goals
- Goals: small footprint, straightforward configuration via env, TLS-PSK support for the service, unit-testable core logic.
- Non-Goals: full HTTP client CLI, advanced routing, or config files beyond env vars.

## Decisions
- Decision: use `actix-web` with OpenSSL for TLS so we can enable PSK via OpenSSL's server callback; configure TLS with env vars.
- Decision: keep the CLI single-purpose (print hello world) to match the requested scope.
- Decision: expose configuration through `HOST` and `PORT`, plus TLS PSK env vars when enabled.

## Risks / Trade-offs
- TLS-PSK support limits framework and TLS stack choices; OpenSSL is required and the implementation targets TLS 1.2 PSK cipher suites.

## Migration Plan
No migration; this is a net-new capability.

## Open Questions
- None.
