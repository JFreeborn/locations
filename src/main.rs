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
    
    
    
    // This is going to act as an Input from external data for now.
    // These values will come from a UI into an API - then flow into this code.
    // For now, they are going to be hardcoded for testing and development.
    // There are going to be multiple flows, eventually. For now a start_ will be called in this file.
    //start_flow_one();
}

/// take in a single lat/long point
/// take in a distance
/// take in a distance unit
/// return the new lat/long point
fn start_flow_one(){
    // Starting lat/long = 34°53'44.9"N 70°54'48.7"E
    /// right now this does not take in the dms, it takes in the decimal variant.
    /// TODO - update to take in either dms or decimal.
    let origin_lat = 34.895808;
    let origin_long = 70.913514;
    let distance = 100.0;
    let unit = "m";
    // The degrees off 0 (north) for the new point
    let bearing = 45.0;

    use_case::flow_one(
        origin_lat,
        origin_long,
        distance,
        unit,
        bearing
    );
}