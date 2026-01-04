## ADDED Requirements
### Requirement: Hello output
The `omnicli` CLI SHALL send `GET /hello` with `text=<arg>` to the service base URL and print the response body to stdout when the request succeeds.

#### Scenario: Default invocation
- **WHEN** the user runs `omnicli hello` with no `OMNI_SERVICE_URL` configured
- **THEN** the CLI sends the request to `http://127.0.0.1:8080/hello?text=hello`
- **AND** stdout contains the response body

#### Scenario: Custom base URL
- **WHEN** `OMNI_SERVICE_URL=http://127.0.0.1:9000` and the user runs `omnicli hello`
- **THEN** the CLI sends the request to `http://127.0.0.1:9000/hello?text=hello`
- **AND** stdout contains the response body

#### Scenario: Missing argument
- **WHEN** the user runs `omnicli` without arguments
- **THEN** the process exits with a non-zero status code

### Requirement: TLS-PSK support
The `omnicli` CLI SHALL use TLS-PSK when `TLS_PSK_ID` and `TLS_PSK` are set and the base URL is HTTPS.

#### Scenario: TLS-PSK enabled
- **WHEN** `OMNI_SERVICE_URL=https://127.0.0.1:8443` and `TLS_PSK_ID`/`TLS_PSK` are set
- **THEN** the CLI authenticates using PSK and prints the response body on success
