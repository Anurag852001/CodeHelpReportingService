use serde::{Deserialize, Serialize};
use crate::r#enum::LoginType::LoginType;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct UserDetails {
    pub(crate) login_id: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) otp_enabled_login: Option<bool>,
    pub(crate) login_type: Option<LoginType>,
}

impl UserDetails {
    pub fn new(login_id: Option<String>, password: Option<String>, otp_enabled_login: Option<bool>, login_type: Option<LoginType>) -> UserDetails {
        UserDetails { login_id, password, otp_enabled_login, login_type }
    }
}