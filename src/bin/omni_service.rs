use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use rust_host_manager::{
    build_psk_acceptor, hello_body, service_version, Config, IdentityResponse, HELLO_PATH,
    IDENTITY_PATH,
};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloQuery {
    text: Option<String>,
}

async fn hello(query: web::Query<HelloQuery>) -> impl Responder {
    match hello_body(query.text.as_deref()) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(body),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

async fn identity() -> impl Responder {
    let response = IdentityResponse {
        service_version: service_version().to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route(HELLO_PATH, web::get().to(hello))
            .route(IDENTITY_PATH, web::get().to(identity))
    });
    let bind_addr = config.bind_addr();

    if let (Some(id), Some(psk)) = (config.tls_psk_id.as_deref(), config.tls_psk.as_deref()) {
        let acceptor = build_psk_acceptor(id, psk)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        server.bind_openssl(bind_addr, acceptor)?.run().await
    } else {
        server.bind(bind_addr)?.run().await
    }
}

fn init_logging() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("service version: {}", service_version());
}

#[cfg(test)]
mod tests {
    use super::init_logging;
    use super::{identity, service_version, IDENTITY_PATH};
    use actix_web::{http::header, App};
    use actix_web::test as actix_test;
    use rust_host_manager::IdentityResponse;
    use serde_json;
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
    fn init_logging_sets_default_rust_log() {
        let _lock = env_lock().lock().expect("env lock");
        let _env = EnvVarGuard::new(&[("RUST_LOG", None)]);
        init_logging();
        assert_eq!(env::var("RUST_LOG").ok().as_deref(), Some("info"));
    }

    #[actix_web::test]
    async fn identity_returns_service_version_json() {
        let app = actix_test::init_service(
            App::new().route(IDENTITY_PATH, actix_web::web::get().to(identity)),
        )
        .await;
        let req = actix_test::TestRequest::get()
            .uri(IDENTITY_PATH)
            .to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let content_type = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok());
        assert_eq!(content_type, Some("application/json"));

        let body = actix_test::read_body(resp).await;
        let parsed: IdentityResponse =
            serde_json::from_slice(&body).expect("identity response json");
        assert_eq!(parsed.service_version, service_version());
    }
}
