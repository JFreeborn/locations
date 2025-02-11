use actix_web::{error, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;

mod use_case;
mod helpers;
mod api;

use crate::api::api::scoped_config;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(move || {
        
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish())
                .into()
            });

        let cors = Cors::default() // Allow requests from any origin
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .max_age(3600); // Cache preflight request for 1 hour

        App::new()
            .wrap(cors)
            .service(web::scope("/api/v1")
                .app_data(json_config)
                .configure(scoped_config))
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}