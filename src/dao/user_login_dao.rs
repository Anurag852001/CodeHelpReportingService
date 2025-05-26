use actix_web::web;
use futures::future::err;
use log::error;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, MySqlPool, Row};
use crate::models::UserDetails::UserDetails;

pub async fn save_user_details(pool:web::Data<MySqlPool>,user_details: UserDetails) -> i64 {
    return sqlx::query("Insert into user_details (login_id,password) values (?,?)")
        .bind(user_details.login_id.unwrap())
        .bind(user_details.password.unwrap())
        .execute(pool.get_ref())
        .await
        .expect("Error while inserting into user_details").last_insert_id() as i64;

}

pub async fn get_user_details_from_db(pool:web::Data<MySqlPool>,login_id:String) -> Result<UserDetails,bool> {
    match sqlx::query("Select * from user_details where login_id = ?").bind(&login_id).fetch_one(pool.get_ref()).await {
        Ok(row) => {
            Ok(UserDetails::new(row.get("login_id"),row.get("password"),None,None))
        }
        Err(e) => {
            error!("Either user details dont exist or some error occured for login_id: {},error:{}",login_id,e);
            Err(false)
        }
    }
}