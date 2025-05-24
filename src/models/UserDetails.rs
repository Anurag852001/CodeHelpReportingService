use serde::{Deserialize, Serialize};
use crate::r#enum::LoginType::LoginType;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct UserDetails {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) otp_enabled_login: bool,
    pub(crate) login_type: LoginType,
}

impl UserDetails {
    pub fn new(email: String, password: String, otp_enabled_login: bool, login_type: LoginType) -> UserDetails {
        UserDetails { email, password,  otp_enabled_login, login_type }
    }
}