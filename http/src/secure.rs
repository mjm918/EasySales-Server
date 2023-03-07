use std::future::IntoFuture;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use actix_cors::Cors;
use actix_web::dev::{Server, ServerHandle};
use actix_web::{App, HttpServer, middleware};
use actix_web::http::header;
use async_trait::async_trait;
use anyhow::{Result,Error};
use log::{debug, info};
use shared::tls::make_config;
use shared::toml_schema::CertConfig;
use shared::util::use_available_threads;
use crate::common::use_default_http_service;
use crate::header::{InternalHttpServer, SecureHttpServer};

#[async_trait]
impl InternalHttpServer<SecureHttpServer> for SecureHttpServer {

    async fn new(address: String, cfg: Option<Arc<CertConfig>>) -> Result<SecureHttpServer, Error> {
        let http_address: SocketAddr = address.parse()?;
        if http_address.ip().to_string() != "" && http_address.port() > 0 && cfg.is_some() {
            let config = cfg.unwrap();
            return Ok(SecureHttpServer{
                address: http_address,
                tls_config: make_config(config.as_ref()),
                sys_config: config.as_ref().clone()
            });
        }
        Err(Error::msg("Http address is not valid"))
    }

    fn start(&self) -> Server {
        debug!("http(tls) server is starting");
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allowed_methods(vec!["GET", "POST", "DELETE"])
                        .allowed_headers(vec![
                            header::AUTHORIZATION,
                            header::ACCEPT,
                            header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        ])
                        .allowed_header(header::CONTENT_TYPE)
                        .supports_credentials()
                        .max_age(3600),
                ).configure(use_default_http_service())
        }).bind_rustls((self.address.ip(), self.address.port()), self.tls_config.as_ref().clone()).unwrap()
            .workers(use_available_threads())
            .run()
    }
}