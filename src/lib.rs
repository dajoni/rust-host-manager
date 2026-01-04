mod api;
mod tls;

use std::env;

pub use api::{
    build_hello_url, build_identity_url, hello_body, IdentityResponse, HELLO_PATH, IDENTITY_PATH,
};
pub use tls::{build_psk_acceptor, psk_https_get};

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub tls_psk_id: Option<String>,
    pub tls_psk: Option<Vec<u8>>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = match env::var("PORT") {
            Ok(value) => value
                .parse::<u16>()
                .map_err(|_| format!("Invalid PORT: {value}"))?,
            Err(_) => 8080,
        };
        let tls_psk_id = env::var("TLS_PSK_ID").ok();
        let tls_psk = env::var("TLS_PSK").ok().map(|value| value.into_bytes());

        let (tls_psk_id, tls_psk) = match (tls_psk_id, tls_psk) {
            (Some(id), Some(psk)) => (Some(id), Some(psk)),
            _ => (None, None),
        };

        Ok(Self {
            host,
            port,
            tls_psk_id,
            tls_psk,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn service_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvVarGuard {
        saved: Vec<(String, Option<String>)>,
    }

    impl EnvVarGuard {
        fn new(vars: &[(&str, Option<&str>)]) -> Self {
            let saved = vars
                .iter()
                .map(|(key, value)| {
                    let previous = env::var(key).ok();
                    match value {
                        Some(val) => env::set_var(key, val),
                        None => env::remove_var(key),
                    }
                    ((*key).to_string(), previous)
                })
                .collect();
            Self { saved }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            for (key, value) in self.saved.drain(..) {
                match value {
                    Some(val) => env::set_var(&key, val),
                    None => env::remove_var(&key),
                }
            }
        }
    }

    #[test]
    fn hello_body_returns_text() {
        let body = hello_body(Some("hello")).expect("expected body");
        assert_eq!(body, "hello");
    }

    #[test]
    fn hello_body_rejects_missing_text() {
        assert!(hello_body(None).is_err());
    }

    #[test]
    fn config_defaults_to_localhost_and_port() {
        let _lock = env_lock().lock().expect("env lock");
        let _env = EnvVarGuard::new(&[
            ("HOST", None),
            ("PORT", None),
            ("TLS_PSK_ID", None),
            ("TLS_PSK", None),
        ]);
        let config = Config::from_env().expect("config");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.tls_psk_id.is_none());
        assert!(config.tls_psk.is_none());
        assert_eq!(config.bind_addr(), "127.0.0.1:8080");
    }

    #[test]
    fn config_uses_env_overrides() {
        let _lock = env_lock().lock().expect("env lock");
        let _env = EnvVarGuard::new(&[
            ("HOST", Some("0.0.0.0")),
            ("PORT", Some("9000")),
            ("TLS_PSK_ID", None),
            ("TLS_PSK", None),
        ]);
        let config = Config::from_env().expect("config");
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9000);
        assert_eq!(config.bind_addr(), "0.0.0.0:9000");
    }

    #[test]
    fn config_requires_both_tls_psk_vars() {
        let _lock = env_lock().lock().expect("env lock");
        let _env = EnvVarGuard::new(&[
            ("HOST", None),
            ("PORT", None),
            ("TLS_PSK_ID", Some("id-only")),
            ("TLS_PSK", None),
        ]);
        let config = Config::from_env().expect("config");
        assert!(config.tls_psk_id.is_none());
        assert!(config.tls_psk.is_none());

        drop(_env);
        let _env = EnvVarGuard::new(&[
            ("HOST", None),
            ("PORT", None),
            ("TLS_PSK_ID", None),
            ("TLS_PSK", Some("psk-only")),
        ]);
        let config = Config::from_env().expect("config");
        assert!(config.tls_psk_id.is_none());
        assert!(config.tls_psk.is_none());

        drop(_env);
        let _env = EnvVarGuard::new(&[
            ("HOST", None),
            ("PORT", None),
            ("TLS_PSK_ID", Some("psk-id")),
            ("TLS_PSK", Some("secret")),
        ]);
        let config = Config::from_env().expect("config");
        assert_eq!(config.tls_psk_id.as_deref(), Some("psk-id"));
        assert_eq!(config.tls_psk.as_deref(), Some("secret".as_bytes()));
    }

    #[test]
    fn service_version_matches_package_version() {
        assert_eq!(service_version(), env!("CARGO_PKG_VERSION"));
    }
}
