use actix_web::web;
use sqlx::MySqlPool;
use crate::models::LoginDetails::LoginDetails;
use crate::models::TokenRequest::TokenRequest;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};

pub trait LoginServiceTrait {
    async fn login(sql_pool:web::Data<MySqlPool>, user_details: UserDetails) ->Result<LoginDetails,String>;
    async fn check_token(token:TokenRequest)->Result<bool,String>;
    async fn verify_otp(sqlPool:web::Data<MySqlPool>,otp: String,session_token:String) -> Result<LoginDetails, String>;
}