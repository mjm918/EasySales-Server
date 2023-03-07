use actix_cors::Cors;
use actix_web::{App, Error, middleware, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::http::header;
use actix_web::web::ServiceConfig;
use shared::util::use_available_threads;

pub fn use_default_http_service() -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::JsonConfig::default().limit(2_097_152 * 100))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }));
    })
}