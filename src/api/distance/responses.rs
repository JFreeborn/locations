use serde::{Deserialize, Serialize};

// For the sake of sanity tonight this is just an easy response
// This should probably live in the application layer
#[derive(Serialize, Deserialize)]
pub struct PointToPointDistanceResponse {
    pub target_lat_short: f64,
    pub target_lon_short: f64,
    pub target_lat_long: f64,
    pub target_lon_long: f64,
    pub message: String
}