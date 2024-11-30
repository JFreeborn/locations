pub mod logic;

pub fn flow_one(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64,
    ) {
    logic::logic_flow_one(
        origin_lat,
        origin_long,
        distance,
        unit,
        bearing);
}