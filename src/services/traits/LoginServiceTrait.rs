use crate::models::LoginDetails::LoginDetails;
use crate::models::UserDetails::UserDetails;

pub trait LoginServiceTrait {
    fn login(user_details: UserDetails)->Result<LoginDetails,String>;
    fn check_token(token:String)->Result<bool,String>;
}