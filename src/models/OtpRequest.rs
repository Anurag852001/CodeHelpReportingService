use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OtpRequest{
    pub(crate) session_token: String,
    pub(crate) otp:String
}