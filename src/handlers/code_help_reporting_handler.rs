
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use crate::services::user_details_service::get_user_details;
use crate::mongo;
use crate::r#enum::Difficulty::Difficulty;
use crate::models::TrackQuestion::TrackQuestion;
use crate::models::UserDetails::UserDetails;
use crate::services::{reporting_service, user_service};

use crate::services::traits::LoginServiceTrait::LoginServiceTrait;
use crate::services::user_service::UserLoginService;

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
    let (uuid, difficulty) = path.into_inner();
    let solved_question =   mongo::mongo_service::calculate_question_solved(uuid.to_string(),Difficulty::from_str(&difficulty.to_string())).await;
    HttpResponse::Ok().json(json!({"solved": solved_question, "uuid": uuid, "question_type": difficulty}))
}


#[post("/track/question")]
pub async fn track_question(payload: web::Json<TrackQuestion>) -> impl Responder {
    let success = reporting_service::track_que(payload.into_inner()).await;
    HttpResponse::Ok().json(json!({"success": success}))
}

#[post("/login")]
pub async fn login(payload: web::Json<UserDetails>) -> impl Responder {
    let user_details:UserDetails = payload.into_inner();
    let result = UserLoginService::login(user_details);

    match result {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(error) => {
            HttpResponse::Ok().json(json!({"success": false, "error": error.to_string()}))
        }
    }

}