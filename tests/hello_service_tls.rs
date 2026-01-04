use actix_web::{web, App, HttpResponse, HttpServer};
use rust_host_manager::{build_psk_acceptor, hello_body, psk_https_get};
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloQuery {
    text: Option<String>,
}

async fn hello(query: web::Query<HelloQuery>) -> HttpResponse {
    match hello_body(query.text.as_deref()) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(body),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_web::test]
async fn hello_over_tls_psk() {
    let id = "test-id";
    let psk = b"test-psk";
    let acceptor = build_psk_acceptor(id, psk).expect("acceptor");

    let server = HttpServer::new(|| App::new().route("/hello", web::get().to(hello)));
    let server = match server.bind_openssl("127.0.0.1:0", acceptor) {
        Ok(server) => server,
        Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("skipping TLS-PSK integration test: {err}");
            return;
        }
        Err(err) => panic!("bind: {err}"),
    };
    let addr = server.addrs()[0];

    let server = server.run();
    let handle = server.handle();
    actix_web::rt::spawn(server);

    let url = format!(
        "https://{}:{}/hello?text=hello",
        addr.ip(),
        addr.port()
    );
    let body = actix_web::rt::task::spawn_blocking(move || psk_https_get(&url, id, psk))
        .await
        .expect("join")
        .expect("psk request");
    assert_eq!(body, "hello");

    handle.stop(true).await;
}
