use crate::models::UserDetails::UserDetails;
use crate::r#enum::LoginType::LoginType;

pub fn get_user_details() ->UserDetails{
    UserDetails {
        email: "johndoe@example.com".to_string(),
        password:"123".to_string(),

        otp_enabled_login: false,
        login_type: LoginType::USER,
    }
}