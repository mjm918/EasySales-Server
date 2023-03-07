use std::future::IntoFuture;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;

use actix_cors::Cors;
use actix_web::{App, dev::ServerHandle, http::header, HttpServer, middleware, web};
use actix_web::dev::Server;
use anyhow::{Result,Error};
use async_trait::async_trait;
use log::{debug, error, info};
use shared::toml_schema::CertConfig;
use shared::util::use_available_threads;
use crate::common::use_default_http_service;
use crate::header::{InsecureHttpServer, InternalHttpServer};

#[async_trait]
impl InternalHttpServer<InsecureHttpServer> for InsecureHttpServer {

    async fn new(address: String, _tls_cfg: Option<Arc<CertConfig>>) -> Result<InsecureHttpServer, Error> {
        let http_address: SocketAddr = address.parse()?;
        if http_address.ip().to_string() != "" && http_address.port() > 0 {
            return Ok(InsecureHttpServer{
                address: http_address
            });
        }
        Err(Error::msg("Http address is not valid"))
    }

     fn start(&self) -> Server {
        debug!("http server is starting");

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
         }).bind((self.address.ip(), self.address.port())).unwrap()
             .workers(use_available_threads()).run()
    }
}