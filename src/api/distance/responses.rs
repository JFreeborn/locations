use serde::{Deserialize, Serialize};

// For the sake of sanity tonight this is just an easy response
// This should probably live in the application layer
#[derive(Serialize, Deserialize)]
pub struct PointToPointDistanceResponse {
    pub target_lat: f64,
    pub target_long: f64,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct Properties {}

#[derive(Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    #[serde(rename = "type")]
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize)]
pub struct GeoJsonResponse {
    #[serde(rename = "type")]
    pub r#type: String,
    pub features: Vec<Struct>,
}