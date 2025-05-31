use std::sync::Arc;
use actix_web::web;
use actix_web::web::get;
use log::{error, info};
use rand::distr::Alphanumeric;
use rand::Rng;
use sqlx::MySqlPool;
use crate::dao::user_login_dao::save_user_details;
use crate::models::LoginDetails::LoginDetails;
use crate::models::TokenRequest::TokenRequest;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};
use crate::services::traits::LoginServiceTrait::LoginServiceTrait;
use crate::services::user_details_service::get_user_details;

pub struct UserLoginService;
pub const LOGIN_TOKEN: &str = "LOGIN_TOKEN";
impl LoginServiceTrait for UserLoginService {

    async fn login(sqlPool:web::Data<MySqlPool>,user_details: UserDetails) -> Result<LoginDetails, String> {
        info!("Received login request :{:?}", user_details);
        if let Some(value) = get_cache(CachingEnums::TwoHours).get(&user_details.login_id.clone().unwrap()).await  {
            return Ok(LoginDetails::new(Option::from(value),true,"Already Logged in".to_string()));
        }
       if let Some(value) =  get_cache(CachingEnums::OneMinute).get(&user_details.login_id.clone().unwrap()).await  {
            return Ok(LoginDetails::new(None,false,"Wait some time before trying again".to_string()));
        }

        //inserting into one minute cache
        get_cache(CachingEnums::OneMinute).insert(user_details.login_id.clone().unwrap(), "true".to_string()).await;
        if user_details.otp_enabled_login.unwrap() {
            let session_token = generate_token();
            sendUserOtp(session_token.clone(),&user_details).await;
            return Ok(LoginDetails::new(Some(session_token), true, "OTP send to email: ".to_string() + &*user_details.login_id.unwrap().to_string()))
        } else {
            //lets check in db if the user exists
            match  get_user_details(sqlPool,user_details.login_id.clone().unwrap()).await {
                Ok(user_details_from_db) => {
                    if(user_details_from_db.password  == None){
                        return Ok(LoginDetails::new(None, true, "Password not found".to_string()));
                    }
                    if(user_details_from_db.password != user_details.password){
                        return Err("Invalid Password".to_string());
                    }
                }
                Err(_) => { return Err("User doesn't exist please sign up".to_string()) }
            };
        }
        let generated_token= generate_token();
        get_cache(CachingEnums::TwoHours).insert(generated_token.clone(), serde_json::to_string(&user_details).unwrap()).await;
        get_cache(CachingEnums::TwoHours).insert(user_details.login_id.unwrap(),generated_token.clone()).await;
        Ok(LoginDetails::new(Some(generated_token), true, "successfully login".to_string()))
    }

     async fn check_token(token_request: TokenRequest) -> Result<bool, String> {
         info!("Received check token :{:?}", token_request);
         let cached_data = get_cache(CachingEnums::TwoHours).get(&token_request.token).await;
         info!("Caching data: {:?}", cached_data);
         if(cached_data.clone() != None) {
            info!("{:?}",cached_data);
            Ok(true)
        } else {
            info!("No token found");
           Err("No token found".to_string())
       }
    }
    async fn verify_otp(sql_pool:web::Data<MySqlPool>, otp: String, session_token:String) -> Result<LoginDetails, String> {
        if otp.trim().eq_ignore_ascii_case("1234")  {
            info!("Received otp {}", session_token);
            if let Some(cached_model) = get_cache(CachingEnums::FiveMins).get(&session_token).await {
                return match serde_json::from_str::<UserDetails>(&cached_model) {
                    //save in db and
                    Ok(user_details) => {
                        save_user_details(sql_pool, user_details).await;
                        Ok(LoginDetails::new(Some(generate_token()), true, "Logged in successfully".to_string()))
                    }
                    Err(_) => { Err("Invalid Otp".to_string()) }
                }
            }
        }
        error!("Invalid OTP: {}", otp);
        Err("Invalid OTP".to_string())
    }
}

fn generate_token() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(20).map(char::from).collect::<String>()
}

async fn sendUserOtp(session_token:String,user_details: &UserDetails) {
    //lets save in cache
    get_cache(CachingEnums::FiveMins).insert(session_token, serde_json::to_string(&user_details.clone()).unwrap()).await;
        //otp sending
}
