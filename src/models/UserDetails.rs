use serde::{Deserialize, Serialize};


#[derive(Serialize)]
#[derive(Deserialize)]
pub struct UserDetails {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) otp_enabled_login: bool,
    pub(crate) is_admin_login: bool,
}

impl UserDetails {
    pub fn new(email: String, password: String, otp_enabled_login: bool, is_admin_login: bool) -> UserDetails {
        UserDetails { email, password,  otp_enabled_login, is_admin_login }
    }
}