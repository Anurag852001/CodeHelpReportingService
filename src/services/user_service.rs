use std::sync::Arc;
use rand::distr::Alphanumeric;
use rand::Rng;
use crate::models::LoginDetails::LoginDetails;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};
use crate::services::traits::LoginServiceTrait::LoginServiceTrait;
use crate::services::user_details_service::get_user_details;

pub struct UserLoginService;
pub const LOGIN_TOKEN: &str = "LOGIN_TOKEN";
impl LoginServiceTrait for UserLoginService {

    async fn login(user_details: UserDetails) -> Result<LoginDetails, String> {
        if(user_details.otp_enabled_login){
            sendUserOtp(&user_details);
            return Ok(LoginDetails::new(None, true, "OTP send to email: ".to_string() + &*user_details.login_id))
        } else {
            //lets check in db if the user exists
            let user_details_from_db = get_user_details(user_details.login_id.clone())
        }
        let generated_token= generate_token(user_details);
        get_cache(CachingEnums::TwoHours).insert(generated_token.clone(), "true".parse().unwrap()).await;
        Ok(LoginDetails::new(Some(generated_token), true, "successfully login".to_string()))
    }

     fn check_token(token: String) -> Result<bool, String> {
        if let Some(cachedData) = get_cache(CachingEnums::TwoHours).get(&(token + LOGIN_TOKEN).to_string()) {
            Ok(true)
        } else {
           Ok(false)
       }
    }
}

async fn verify_otp(otp: String, id: i64) -> Result<LoginDetails, String> {
    if (otp.eq_ignore_ascii_case("1234")) {
        if let Some(cached_model) = get_cache(CachingEnums::TwoHours).get(&id.to_string()).await {
            return match serde_json::from_str::<UserDetails>(&cached_model) {
                //save in db and
                Ok(user_details) => { Ok(LoginDetails::new(generate_token(user_details), true, "Logged in successfully".to_string())) }
                Err(_) => { Err("Error while logging in".to_string()) }
            }
        }
    }
    Err("Invalid OTP".to_string())
}

fn generate_token(user_details: UserDetails) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(20).map(char::from).collect::<String>()
}

fn sendUserOtp(user_details: &UserDetails) {

}
