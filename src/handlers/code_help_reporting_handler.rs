use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use crate::services::user_details_service::get_user_details;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": format!("Hello, {}!", name)
    }))
}

#[get("/test/{name}")]
pub async fn test(name: web::Path<String>) -> impl Responder {
    let user_details = get_user_details();
    HttpResponse::Ok().json(user_details)
}