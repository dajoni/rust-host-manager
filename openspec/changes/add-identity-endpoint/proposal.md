# Change: Add service identity endpoint

## Why
Clients need a stable way to discover the running service and API versions for compatibility checks.

## What Changes
- Move all existing endpoints under `/v1/`.
- Add a new identity endpoint at `/v1/identity` that returns service versions.
- Return the version of the running binary in the response.

## Impact
- Affected specs: service-identity
- Affected code: API handlers, service binary, API library
