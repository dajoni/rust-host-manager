# Rust Host Manager

[![tests](https://github.com/dajoni/rust-host-manager/actions/workflows/tests.yml/badge.svg)](https://github.com/dajoni/rust-host-manager/actions/workflows/tests.yml)

## Run the service (no TLS)

```bash
cargo run --bin omni-service
```

The service binds to `127.0.0.1:8080` by default. Override with:

```bash
HOST=0.0.0.0 PORT=9000 cargo run --bin omni-service
```

Example request:

```bash
curl "http://127.0.0.1:8080/v1/hello?text=hello"
```

Identity request:

```bash
curl "http://127.0.0.1:8080/v1/identity"
```

## Run the service (TLS-PSK)

TLS-PSK is enabled when both `TLS_PSK_ID` and `TLS_PSK` are set. The PSK value is used as raw bytes from the environment variable.

```bash
HOST=127.0.0.1 PORT=8443 TLS_PSK_ID=client TLS_PSK=secret cargo run --bin omni-service
```

## Run the CLI (REST client)

```bash
cargo run --bin omnicli -- hello "Hello, world"
```

Fetch the service version:

```bash
cargo run --bin omnicli -- identity
```

Set a custom service URL with:

```bash
OMNI_SERVICE_URL=http://127.0.0.1:8080 cargo run --bin omnicli -- hello "Hello, world"
```

PSK-enabled requests require HTTPS plus `TLS_PSK_ID` and `TLS_PSK`:

```bash
OMNI_SERVICE_URL=https://127.0.0.1:8443 TLS_PSK_ID=client TLS_PSK=secret cargo run --bin omnicli -- hello "Hello, world"
```
