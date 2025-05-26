use actix_web::{web};
use crate::handlers::code_help_reporting_handler;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/codehelp")
            .service(code_help_reporting_handler::greet)
            .service(code_help_reporting_handler::question_solved_type_solved)
            .service(code_help_reporting_handler::track_question)
            .service(code_help_reporting_handler::login)
            .service(code_help_reporting_handler::verify_otp)
        );
}
