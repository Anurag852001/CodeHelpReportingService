use actix_web::{web};
use crate::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/codehelp")
            .service(handlers::code_help_reporting_handler::greet)
            .service(handlers::code_help_reporting_handler::test)
        );
}
