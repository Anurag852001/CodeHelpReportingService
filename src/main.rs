mod handlers;
mod middlewares;
mod routes;
mod utils;
mod services;
mod models;
mod mongo;
mod r#enum;
mod dao;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use log::info;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use crate::services::tracker_service::init_question_tracker;

#[actix_web::main] // or #[tokio::main]
    async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none(){
            std::env::set_var("RUST_LOG", "info");
        }
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = web::Data::new(MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&utils::constants::MYSQL_URI)
        .await.unwrap());


    let port = utils::constants::PORT.clone();
    let address = utils::constants::ADDRESS.clone();
        init_question_tracker().await;
        HttpServer::new(move || {
            App::new().wrap(Cors::default().allow_any_header().allow_any_method().allow_any_origin() )
                .wrap(Logger::default())
                .configure(routes::code_help_reporting_routes::config)
                .app_data(pool.clone())



        })
            .bind((address, port))?
            .run()
            .await

    }


