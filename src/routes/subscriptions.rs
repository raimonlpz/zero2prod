use actix_web::web;
use actix_web::HttpResponse;

use super::FormData;

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
