use actix_web::{web, get, Responder, HttpResponse};
use actix_web::web::resource;
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

    match logic::get_basic_distance(
            query.origin_lat,
            query.origin_long,
            query.distance,
            &query.unit,
            query.bearing) {
        Ok(success) => HttpResponse::Ok().json(success),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error {}", err))
    }
}

#[get("/get_geo_json")]
pub async fn get_geo_json(query: web::Query<PointToPointDistanceRequest>) -> impl Responder {

    if query.distance.is_nan() || query.bearing.is_nan() {
        return HttpResponse::BadRequest().body("Invalid numeric input");
    }

    match logic::get_geo_json(
        query.origin_lat,
        query.origin_long,
        query.distance,
        &query.unit,
        query.bearing) {
        Ok(success) => HttpResponse::Ok().json(success),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error {}", err))
    }
}