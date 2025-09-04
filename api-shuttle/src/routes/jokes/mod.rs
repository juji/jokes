pub mod retrieve;
pub mod random;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jokes")
            .service(retrieve::retrieve_jokes)
            .service(random::random_joke)
    );
}
