use actix_web::web;
use sqlx::MySqlPool;
use crate::dao::user_login_dao::get_user_details_from_db;
use crate::models::UserDetails::UserDetails;


pub async fn get_user_details(sqlPool:web::Data<MySqlPool>, login_id:String) -> Result<UserDetails,bool>{
    return get_user_details_from_db(sqlPool,login_id).await
}