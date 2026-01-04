use rust_host_manager::{build_hello_url, build_identity_url, psk_https_get, IdentityResponse};
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(value) => value,
        None => {
            eprintln!("Usage: omnicli hello <text>\n       omnicli identity");
            std::process::exit(1);
        }
    };

    let base_url =
        env::var("OMNI_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let tls_psk_id = env::var("TLS_PSK_ID").ok();
    let tls_psk = env::var("TLS_PSK").ok().map(|value| value.into_bytes());

    if command == "identity" {
        if args.next().is_some() {
            eprintln!("Usage: omnicli identity");
            std::process::exit(1);
        }
        let url = match build_identity_url(&base_url) {
            Ok(url) => url,
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(1);
            }
        };
        let body = fetch_body(&url, tls_psk_id.as_deref(), tls_psk.as_deref());
        let identity: IdentityResponse = match serde_json::from_str(&body) {
            Ok(identity) => identity,
            Err(err) => {
                eprintln!("Failed parsing identity response: {err}");
                std::process::exit(1);
            }
        };
        println!("{}", identity.service_version);
    } else if command == "hello" {
        let text = match args.next() {
            Some(value) => value,
            None => {
                eprintln!("Usage: omnicli hello <text>");
                std::process::exit(1);
            }
        };
        if args.next().is_some() {
            eprintln!("Usage: omnicli hello <text>");
            std::process::exit(1);
        }
        let url = match build_hello_url(&base_url, &text) {
            Ok(url) => url,
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(1);
            }
        };
        let body = fetch_body(&url, tls_psk_id.as_deref(), tls_psk.as_deref());
        println!("{body}");
    } else {
        eprintln!("Unknown command: {command}");
        eprintln!("Usage: omnicli hello <text>\n       omnicli identity");
        std::process::exit(1);
    }
}

fn fetch_body(url: &str, tls_psk_id: Option<&str>, tls_psk: Option<&[u8]>) -> String {
    if let (Some(id), Some(psk)) = (tls_psk_id, tls_psk) {
        match psk_https_get(url, id, psk) {
            Ok(body) => body,
            Err(err) => {
                eprintln!("Request failed: {err}");
                std::process::exit(1);
            }
        }
    } else {
        let response = match reqwest::blocking::get(url) {
            Ok(response) => response,
            Err(err) => {
                eprintln!("Request failed: {err}");
                std::process::exit(1);
            }
        };

        if !response.status().is_success() {
            eprintln!("Request failed with status {}", response.status());
            std::process::exit(1);
        }

        match response.text() {
            Ok(body) => body,
            Err(err) => {
                eprintln!("Failed reading response: {err}");
                std::process::exit(1);
            }
        }
    }
}
