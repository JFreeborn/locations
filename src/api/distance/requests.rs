use serde::Deserialize;

#[derive(Deserialize)]
pub struct PointToPointDistanceRequest {
    pub origin_lat: f64,
    pub origin_long: f64,
    pub distance: f64,
    pub unit: String,
    pub bearing: f64,
}