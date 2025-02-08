mod handlers;
mod middlewares;
mod routes;
mod utils;
mod services;
mod r#struct;
mod mongo;
mod r#enum;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, Responder};


#[actix_web::main] // or #[tokio::main]
    async fn main() -> std::io::Result<()> {
        let port = utils::constants::PORT.clone();
        let address = utils::constants::ADDRESS.clone();
        if std::env::var_os("RUST_LOG").is_none(){
            std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
        }
        dotenv::dotenv().ok();
        env_logger::init();
        HttpServer::new(|| {
            App::new().wrap(Cors::default().allow_any_header().allow_any_method().allow_any_origin() ).wrap(Logger::default()).configure(routes::code_help_reporting_routes::config)


        })
            .bind((address, port))?
            .run()
            .await
    }


