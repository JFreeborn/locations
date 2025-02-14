use crate::api;
use actix_web::{web};

/*
this registers all the endpoints and makes them available
to the main.rs file for registration
*/
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(api::distance::endpoints::first_endpoint)
        .service(api::distance::endpoints::test_route)
    ;
}