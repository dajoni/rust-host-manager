## ADDED Requirements
### Requirement: Service identity endpoint
The service SHALL expose a `GET /v1/identity` endpoint that returns JSON containing the service version. The service version MUST match the version used when tagging the `main` branch release. The endpoint MUST follow the same TLS-PSK security requirements as other service endpoints.

#### Scenario: Identity request succeeds
- **WHEN** a client sends `GET /v1/identity`
- **THEN** the response status is 200
- **AND** the response body is JSON with a `service_version` field

### Requirement: Version is derived from Cargo metadata
The service version SHALL be sourced from Cargo package metadata so that it reflects the tagged release version without manual updates.

#### Scenario: Version syncs with release tag
- **WHEN** the service binary is built for a tagged release
- **THEN** the `service_version` matches the Cargo package version used for that tag

### Requirement: Endpoints must be versioned
The service SHALL expose endpoints under a `/v<number>/` path, starting with `/v1/` for backward compatibility reasons.

#### Scenario: Clients use versioned path
- **WHEN** a client sends a request to a service endpoint
- **THEN** the request path starts with `/v1/`
