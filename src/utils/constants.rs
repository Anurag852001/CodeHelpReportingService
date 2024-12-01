use std::env;
use lazy_static::lazy_static;
use log::{error, info};

lazy_static!{
     pub static ref ADDRESS:String = set_address();
     pub static ref PORT:u16 = set_port();
}

fn set_address() -> String {
     match dotenv::dotenv() {
          Ok(_) => info!("Loaded .env file successfully."),
          Err(err) => error!("Failed to load .env file: {}", err)
     }
     env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
     match dotenv::dotenv() {
          Ok(_) => info!("Loaded .env file successfully."),
          Err(err) => error!("Failed to load .env file: {}", err)
     }
     env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok()).unwrap()
}