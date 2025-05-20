use log::error;
use crate::models::LoginDetails::LoginDetails;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};

pub fn login(user_details: UserDetails) -> LoginDetails {

}


pub fn verifyOtp(otp: String,id:i64) -> LoginDetails {
    if(otp.eq_ignore_ascii_case("1234")){
       if let Some (cached_model) = get_cache(CachingEnums::FiveMins).get(&id.to_string()) {
           return if let Some(user_details) = serde_json::from_str::<UserDetails>(&cached_model) {
               //save in db and
                LoginDetails::
           } else {
               return "Error while verifying otp".to_string()
           }
       }
    }
    "Invalid OTP".to_string()
}

pub fn generate_token(user_details: UserDetails) -> String {

}