use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use log::info;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    for  i in 1..=10 {
        print!("{}", i);
    }
    HttpResponse::Ok().json(json!({
        "message": format!("Hello, {}!", name)
    }))
}

    #[actix_web::main] // or #[tokio::main]
    async fn main() -> std::io::Result<()> {
        for  i in 1..=10 {
            println!("{}", i);
        }
        if std::env::var_os("RUST_LOG").is_none(){
            std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
        }
        dotenv::dotenv().ok();
        env_logger::init();
        HttpServer::new(|| {
            App::new().wrap(Logger::default()).service(greet)

        })
            .bind(("127.0.0.1", 9000))?
            .run()
            .await
    }


