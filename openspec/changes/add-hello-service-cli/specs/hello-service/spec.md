## ADDED Requirements
### Requirement: Hello endpoint
The service SHALL expose `GET /hello` and return the exact value of the `text` query parameter as the response body using `text/plain; charset=utf-8`.

#### Scenario: Request includes text
- **WHEN** the client sends `GET /hello?text=hello`
- **THEN** the response status is 200
- **AND** the response body is `hello`

#### Scenario: Request omits text
- **WHEN** the client sends `GET /hello` without `text`
- **THEN** the response status is 400

### Requirement: Bind configuration
The service SHALL bind to a host and port defined by `HOST` and `PORT` environment variables and SHALL default to `127.0.0.1:8080` when they are not set.

#### Scenario: Defaults apply
- **WHEN** `HOST` and `PORT` are unset
- **THEN** the service binds to `127.0.0.1:8080`

#### Scenario: Environment overrides
- **WHEN** `HOST=0.0.0.0` and `PORT=9000` are set
- **THEN** the service binds to `0.0.0.0:9000`

### Requirement: TLS-PSK support
The service SHALL support TLS with pre-shared keys when `TLS_PSK_ID` and `TLS_PSK` environment variables are set.

#### Scenario: TLS-PSK enabled
- **WHEN** `TLS_PSK_ID` and `TLS_PSK` are set
- **THEN** the service accepts TLS connections using PSK authentication

#### Scenario: TLS-PSK disabled
- **WHEN** `TLS_PSK_ID` or `TLS_PSK` is missing
- **THEN** the service runs without TLS

### Requirement: Request logging
The service SHALL log each HTTP request with method, path, response status, and latency.

#### Scenario: Request handled
- **WHEN** the service handles a request
- **THEN** a log entry is emitted including the request method, path, response status, and elapsed time
