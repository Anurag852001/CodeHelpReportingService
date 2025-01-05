use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use crate::services::user_details_service::get_user_details;
use crate::mongo;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    mongo::mongo_service::test_mongo().await.expect("TODO: panic message");
    HttpResponse::Ok().json(json!({
        "message": format!("Hello, {}!", name)
    }))
}

#[get("/test/{name}")]
pub async fn test(name: web::Path<String>) -> impl Responder {
    let user_details = get_user_details();
    HttpResponse::Ok().json(user_details)
}