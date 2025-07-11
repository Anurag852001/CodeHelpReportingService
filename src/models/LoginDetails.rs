use serde::Serialize;

#[derive(Serialize)]
pub struct LoginDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) token: Option<String>,
    pub(crate) success: bool,
    pub(crate) message: String,
}

impl crate::models::LoginDetails::LoginDetails {
    pub fn new(token:Option<String>, success:bool, message:String) -> LoginDetails {
       LoginDetails{token,success,message}
    }
}