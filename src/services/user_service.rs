use rand::distr::Alphanumeric;
use rand::Rng;
use crate::models::LoginDetails::LoginDetails;
use crate::models::UserDetails::UserDetails;
use crate::r#enum::CachingEnums::{get_cache, CachingEnums};
use crate::utils::constants::MONGO_URI;

pub fn login(user_details: UserDetails) -> Result<LoginDetails,String> {

}


pub fn verifyOtp(otp: String,id:i64) -> Result<LoginDetails,String> {
    if(otp.eq_ignore_ascii_case("1234")){
       if let Some (cached_model) = get_cache(CachingEnums::FiveMins).get(&id.to_string()) {
           return if let Some(user_details) = serde_json::from_str::<UserDetails>(&cached_model) {
               //save in db and
               Ok(LoginDetails::new(generate_token(), true, "Logged in successfully".to_string()))
           } else {
               return Err("Error while logging in".to_string())
           }
       }
    }
    Err("Invalid OTP".to_string())
}

fn generate_token(user_details: UserDetails) -> String {
    let token = rand::thread_rng().sample_iter(&Alphanumeric).take(20).map(char::from).collect();
    get_cache(CachingEnums::TwoHours).insert(user_details.email + "Login Otp",token);
    //save in db too we dont have aerospike
    token
}