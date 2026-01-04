use rust_host_manager::{build_hello_url, psk_https_get};
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let text = match args.next() {
        Some(value) => value,
        None => {
            eprintln!("Usage: omnicli <text>");
            std::process::exit(1);
        }
    };

    let base_url =
        env::var("OMNI_SERVICE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let url = match build_hello_url(&base_url, &text) {
        Ok(url) => url,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    let tls_psk_id = env::var("TLS_PSK_ID").ok();
    let tls_psk = env::var("TLS_PSK").ok().map(|value| value.into_bytes());

    let body = if let (Some(id), Some(psk)) = (tls_psk_id, tls_psk) {
        match psk_https_get(&url, &id, &psk) {
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
    };

    println!("{body}");
}
