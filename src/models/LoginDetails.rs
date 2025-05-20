use serde::Serialize;

#[derive(Serialize)]
pub struct LoginDetails {
    pub(crate) token: String,
    pub(crate) success: bool,
    pub(crate) message: String,
}

impl crate::models::UserDetails::UserDetails {
    pub fn new(token:String, success:bool, message:String) -> LoginDetails {
       LoginDetails{token,success,message}
    }
}