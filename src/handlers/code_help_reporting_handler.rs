use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::Path;
use serde_json::json;
use crate::services::user_details_service::get_user_details;
use crate::mongo;
use crate::r#enum::question_type_enum::QuestionType;

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

#[get("/questions_solved/{uuid}/{question_type}")]
pub async fn question_solved_type_solved(path:web::Path<(String,String)>) -> impl Responder {
    let (uuid, question_type) = path.into_inner();
    let solved_question =   mongo::mongo_service::calculate_question_solved(uuid.to_string(),QuestionType::from_str(&question_type.to_string())).await;
    HttpResponse::Ok().json(json!({"solved": solved_question, "uuid": uuid, "question_type": question_type}))
}