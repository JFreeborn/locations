pub mod api {

    use actix_web::{web, get, Responder, HttpResponse};
    use serde::Deserialize;
    use serde_json::json;
    use crate::use_case::flow_one;


    #[derive(Deserialize)]
    pub struct FlowRequest {
        origin_lat: f64,
        origin_long: f64,
        distance: f64,
        unit: String,
        bearing: f64,
    }

    #[get("/info")]
    async fn test_route() -> impl Responder {
        HttpResponse::Ok().body("Ok")
    }

    #[get("/first_endpoint")]
    async fn first_endpoint(query: web::Query<FlowRequest>) -> impl Responder {

        if query.distance.is_nan() || query.bearing.is_nan() {
            return HttpResponse::BadRequest().body("Invalid numeric input");
        }

        let (target_lat_short, target_lon_short, target_lat_long, target_lon_long, formatted_dms) =
             flow_one(
                query.origin_lat,
                query.origin_long,
                query.distance,
                &query.unit,
                query.bearing,
            );

        HttpResponse::Ok().json(json!({
            "target_lat_short": target_lat_short,
            "target_lon_short": target_lon_short,
            "target_lat_long": target_lat_long,
            "target_lon_long": target_lon_long,
            "formatted_dms": formatted_dms
        }))
    }

    pub fn scoped_config(cfg: &mut web::ServiceConfig) {
        cfg
            .service(first_endpoint)
            .service(test_route)
        ;
    }
}
