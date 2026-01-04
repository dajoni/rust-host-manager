# Change: Add hello REST service and CLI

## Why
We need a minimal Rust REST service that returns a caller-provided string and a simple CLI that can produce a hello-world output. TLS-PSK support is required for deployments that need shared-key transport security.

## What Changes
- Add a REST service with `GET /hello` that returns the `text` query parameter as the response body.
- Add a separate `omnicli` CLI binary that calls the service and prints the response body.
- Add TLS-PSK support for the REST service, configurable via environment variables.
- Add environment-based configuration for host and port with sensible defaults.
- Add unit tests covering the handler and CLI output.

## Impact
- Affected specs: `hello-service`, `hello-cli`
- Affected code: new Rust service and CLI binaries, shared configuration module
