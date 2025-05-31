use std::future::Future;
use std::result;
use actix_web::{get, options, post, web, HttpResponse, Responder};
use actix_web::dev::JsonBody;
use actix_web::web::service;
use serde_json::json;
use sqlx::MySqlPool;
use crate::models::LoginDetails::LoginDetails;
use crate::models::OtpRequest::OtpRequest;
use crate::models::TokenRequest::TokenRequest;
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
pub async fn test(sqlPool:web::Data<MySqlPool>,name: web::Path<String>) -> impl Responder {
    let user_details = get_user_details(sqlPool,"".parse().unwrap());
    HttpResponse::Ok().json(user_details.await)
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
pub async fn login(sqlPool:web::Data<MySqlPool>,payload: web::Json<UserDetails>) -> impl Responder {
    let user_details:UserDetails = payload.into_inner();
    let result = UserLoginService::login(sqlPool,user_details);

    match result.await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(error) => {
            HttpResponse::Ok().json(json!({"success": false, "error": error.to_string()}))
        }
    }

}

#[post("/verify/otp")]
pub async fn verify_otp(sqlPool:web::Data<MySqlPool>,json_body: String) -> impl Responder {
    let otp_request:OtpRequest = serde_json::from_str(&json_body).unwrap();
    let result = UserLoginService::verify_otp(sqlPool, otp_request.otp, otp_request.session_token).await;
    match result {
        Ok(login_details)=>{
            HttpResponse::Ok().json(login_details)
        }
        Err(error)=>{
            HttpResponse::Ok().json(json!({"success":false,"error":error}))
        }
    }
}

#[options("/{_:.*}")]
pub async fn options() -> impl Responder {
    HttpResponse::Ok()
}


#[post("check/token")]
pub async fn check_token(token_request:web::Json<TokenRequest>) -> impl Responder {
       match UserLoginService::check_token(token_request.into_inner()).await {
           Ok(_) => {HttpResponse::Ok().json(json!({"success": true,"message":"Already logged in."}))}
           Err(_) => {HttpResponse::Ok().json(json!({"success": false, "message": "Token expired or invalid"}))}
       }
}