use rand::distr::Alphanumeric;
use rand::Rng;
use crate::models::LoginDetails::LoginDetails;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};
use crate::services::traits::LoginServiceTrait::LoginServiceTrait;

pub struct UserLoginService;

impl LoginServiceTrait for UserLoginService {
    fn login(user_details: UserDetails) -> Result<LoginDetails, String> {
        Ok(LoginDetails::new(generate_token(user_details), true, "successfully login".to_string()))
    }

}

async fn verifyOtp(otp: String, id: i64) -> Result<LoginDetails, String> {
    if (otp.eq_ignore_ascii_case("1234")) {
        if let Some(cached_model) = get_cache(CachingEnums::FiveMins).get(&id.to_string()).await {
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
