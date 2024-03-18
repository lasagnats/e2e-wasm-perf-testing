use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Serialize};

mod api;
mod services;


#[derive(Serialize)]
pub struct Response {
    pub message: String,
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move ||
        App::new()
            .configure(api::api::config)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
