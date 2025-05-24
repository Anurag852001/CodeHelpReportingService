
use crate::dao::user_login_dao::get_user_details_from_db;
use crate::models::UserDetails::UserDetails;


pub fn get_user_details(login_id:&str) -> Result<UserDetails,bool>{
    let pool;
    return get_user_details_from_db(pool,login_id)
}