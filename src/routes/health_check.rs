use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
