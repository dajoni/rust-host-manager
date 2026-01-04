use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslConnector, SslMethod, SslVersion};
use reqwest::Url;
use std::io::{Read, Write};

pub fn build_psk_acceptor(
    id: &str,
    psk: &[u8],
) -> Result<SslAcceptorBuilder, openssl::error::ErrorStack> {
    let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server())?;
    builder.set_min_proto_version(Some(SslVersion::TLS1_2))?;
    builder.set_max_proto_version(Some(SslVersion::TLS1_2))?;
    builder.set_cipher_list("PSK")?;

    let expected_id = id.as_bytes().to_vec();
    let expected_psk = psk.to_vec();
    builder.set_psk_server_callback(move |_ssl, identity, psk_buf| {
        let identity = match identity {
            Some(value) => value,
            None => return Ok(0),
        };
        if identity != expected_id.as_slice() {
            return Ok(0);
        }
        let len = expected_psk.len();
        if psk_buf.len() < len {
            return Ok(0);
        }
        psk_buf[..len].copy_from_slice(&expected_psk);
        Ok(len)
    });

    Ok(builder)
}

pub fn build_psk_connector(
    id: &str,
    psk: &[u8],
) -> Result<SslConnector, openssl::error::ErrorStack> {
    let mut builder = SslConnector::builder(SslMethod::tls_client())?;
    builder.set_min_proto_version(Some(SslVersion::TLS1_2))?;
    builder.set_max_proto_version(Some(SslVersion::TLS1_2))?;
    builder.set_cipher_list("PSK")?;

    let id_bytes = id.as_bytes().to_vec();
    let psk_bytes = psk.to_vec();
    builder.set_psk_client_callback(move |_ssl, _hint, identity, psk_buf| {
        if identity.len() < id_bytes.len() + 1 {
            return Ok(0);
        }
        identity[..id_bytes.len()].copy_from_slice(&id_bytes);
        identity[id_bytes.len()] = 0;

        if psk_buf.len() < psk_bytes.len() {
            return Ok(0);
        }
        psk_buf[..psk_bytes.len()].copy_from_slice(&psk_bytes);
        Ok(psk_bytes.len())
    });

    Ok(builder.build())
}

pub fn psk_https_get(url: &str, id: &str, psk: &[u8]) -> Result<String, String> {
    let parsed = Url::parse(url).map_err(|err| err.to_string())?;
    if parsed.scheme() != "https" {
        return Err("PSK requests require an https URL".to_string());
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| "Missing host".to_string())?;
    let port = parsed
        .port_or_known_default()
        .ok_or_else(|| "Missing port".to_string())?;

    let addr = format!("{}:{}", host, port);
    let stream = std::net::TcpStream::connect(addr).map_err(|err| err.to_string())?;

    let connector = build_psk_connector(id, psk).map_err(|err| err.to_string())?;
    let mut stream = connector
        .connect(host, stream)
        .map_err(|err| err.to_string())?;

    let mut path = parsed.path().to_string();
    if let Some(query) = parsed.query() {
        path.push('?');
        path.push_str(query);
    }

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|err| err.to_string())?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|err| err.to_string())?;

    let body = response
        .split("\r\n\r\n")
        .nth(1)
        .ok_or_else(|| "Missing response body".to_string())?;

    Ok(body.to_string())
}
