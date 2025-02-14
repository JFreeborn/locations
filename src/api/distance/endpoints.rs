use actix_web::{web, get, Responder, HttpResponse};
use super::requests::PointToPointDistanceRequest;
use super::responses::PointToPointDistanceResponse;

use crate::application::distance_use_case::logic;
// This is a dummy endpoint
#[get("/info")]
pub async fn test_route() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/first_endpoint")]
pub async fn first_endpoint(query: web::Query<PointToPointDistanceRequest>) -> impl Responder {

    if query.distance.is_nan() || query.bearing.is_nan() {
        return HttpResponse::BadRequest().body("Invalid numeric input");
    }

    let (target_lat_short, target_lon_short, target_lat_long, target_lon_long, formatted_dms) =
        logic::logic_flow_one(
            query.origin_lat,
            query.origin_long,
            query.distance,
            &query.unit,
            query.bearing,
        );

    let fart = PointToPointDistanceResponse{
        message: "here is some string".into()
    };

    HttpResponse::Ok().json(fart)
}