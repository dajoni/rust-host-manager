use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rust_host_manager::{build_hello_url, build_psk_acceptor, hello_body, psk_https_get, HELLO_PATH};
use serde::Deserialize;
use std::net::TcpListener;
use std::time::Duration;

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

#[actix_web::test]
async fn service_without_psk() {
    let (base_url, handle) = spawn_plain_server().await;
    let url = build_hello_url(&base_url, "hello").expect("url");

    let response = reqwest::get(url).await.expect("request");
    assert!(response.status().is_success());
    let body = response.text().await.expect("body");
    handle.stop(true).await;
    assert_eq!(body, "hello");
}

#[actix_web::test]
async fn service_with_psk() {
    let (base_url, handle) = spawn_psk_server("client", b"secret").await;
    let body = psk_get(&base_url, "hello", "client", b"secret").expect("psk request");
    handle.stop(true).await;
    assert_eq!(body, "hello");
}

async fn spawn_psk_server(
    psk_id: &str,
    psk: &[u8],
) -> (String, actix_web::dev::ServerHandle) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("local addr");
    let acceptor = build_psk_acceptor(psk_id, psk).expect("acceptor");

    let server = HttpServer::new(|| App::new().route(HELLO_PATH, web::get().to(hello)))
        .listen_openssl(listener, acceptor)
        .expect("listen")
        .run();
    let handle = server.handle();
    actix_web::rt::spawn(server);

    actix_web::rt::time::sleep(Duration::from_millis(50)).await;
    (format!("https://{}", addr), handle)
}

async fn spawn_plain_server() -> (String, actix_web::dev::ServerHandle) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind listener");
    let addr = listener.local_addr().expect("local addr");

    let server = HttpServer::new(|| App::new().route(HELLO_PATH, web::get().to(hello)))
        .listen(listener)
        .expect("listen")
        .run();
    let handle = server.handle();
    actix_web::rt::spawn(server);

    actix_web::rt::time::sleep(Duration::from_millis(50)).await;
    (format!("http://{}", addr), handle)
}

fn psk_get(base_url: &str, text: &str, id: &str, psk: &[u8]) -> Result<String, String> {
    let url = build_hello_url(base_url, text)?;
    psk_https_get(&url, id, psk)
}
