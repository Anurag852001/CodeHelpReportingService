use actix_web::{web};
use crate::handlers::code_help_reporting_handler;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/codehelp")
            .service(code_help_reporting_handler::greet)
            .service(code_help_reporting_handler::test)
            .service(code_help_reporting_handler::question_solved_type_solved)
        );
}
