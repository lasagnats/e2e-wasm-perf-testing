use crate::services::generator::generate_entries;
use actix_web::{
    get,
    http::header::{self, HeaderValue},
    web, HttpResponse,
};

#[get("/data/{count}")]
pub async fn get_entries(count: web::Path<u32>) -> HttpResponse {
    let count_value = count.into_inner();
    let entries = generate_entries(count_value);
    if entries.is_empty() {
        HttpResponse::NotFound().body("Entries could not be fetched")
    } else {
        let mut res = HttpResponse::Ok().json(entries);
        let headers = res.headers_mut();
        headers.append(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );

        res
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(get_entries));
}
