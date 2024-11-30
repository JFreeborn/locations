use std::f64::consts::PI;


/// These helpers should likely be in their own are of the repo
/// Function to convert degrees to radians
fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Function to convert radians to degrees
fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / PI
}

fn decimal_to_dms(decimal: f64) -> (i32, i32, f64) {
    let degrees = decimal.trunc() as i32; // Integer part
    let minutes_decimal = (decimal.abs() - degrees.abs() as f64) * 60.0;
    let minutes = minutes_decimal.trunc() as i32; // Integer part of minutes
    let seconds = (minutes_decimal - minutes as f64) * 60.0; // Remainder as seconds
    (degrees, minutes, seconds)
}

pub fn logic_flow_one(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64,) {
    // for now, we are ignoring the unit type and are only supporting meters
    // This needs to take in the distance and determine which measuring method to use.
    // this version does not need to be used for this short of a distance
    const EARTH_RADIUS: f64 = 6371000.0; // Earth's radius in meters

    // Convert input degrees to radians
    let start_lat_rad = deg_to_rad(origin_lat);
    let start_lon_rad = deg_to_rad(origin_long);
    let bearing_rad = deg_to_rad(bearing);

    // Calculate the destination latitude
    let angular_distance = distance / EARTH_RADIUS;
    let lat2_rad = (start_lat_rad.sin() * angular_distance.cos()
        + start_lat_rad.cos() * angular_distance.sin() * bearing_rad.cos())
        .asin();

    // Calculate the destination longitude
    let lon2_rad = start_lon_rad
        + (bearing_rad.sin() * angular_distance.sin() * start_lat_rad.cos())
        .atan2(angular_distance.cos() - start_lat_rad.sin() * lat2_rad.sin());

    // Convert radians back to degrees
    let lat2 = rad_to_deg(lat2_rad);
    let lon2 = rad_to_deg(lon2_rad);

    println!("Destination Latitude: {:.6}", lat2);
    println!("Destination Longitude: {:.6}", lon2);

    println!("combo: {:.6}, {:.6}", lat2, lon2);

    let (lat_deg, lat_min, lat_sec) = decimal_to_dms(lat2);
    let (lon_deg, lon_min, lon_sec) = decimal_to_dms(lon2);

    let target_lat_degrees = lat_deg.abs();
    let target_lat_min = lat_min;
    let target_lat_sec = lat_sec;
    let target_lat_loc = if lat2 >= 0.0 {"N"} else {"S"};
    let formatted_lat = format!(
        "{}°{}'{:.1}\"{}",
        target_lat_degrees,
        target_lat_min,
        target_lat_sec,
        target_lat_loc
    );

    let target_lon_degrees = lon_deg.abs();
    let target_lon_min = lon_min;
    let target_lon_sec = lon_sec;
    let target_lon_loc = if lon2 >= 0.0 {"E"} else {"W"};
    let formatted_lon = format!(
        "{}°{}'{:.1}\"{}",
        target_lon_degrees,
        target_lon_min,
        target_lon_sec,
        target_lon_loc
    );

    println!("Formatted target location: {} {}", formatted_lat, formatted_lon);
}
