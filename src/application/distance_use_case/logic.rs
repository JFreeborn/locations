use super::helpers::math::{deg_to_rad, rad_to_deg};
use super::helpers::format::{return_dms_from_lat_long};
use crate::api::distance::responses::PointToPointDistanceResponse;
use std::io::{Error, ErrorKind};

fn calculate_distance_short(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64
) -> (f64, f64) {
    // TODO - will need to convert the distance_use_case to meters before calling this function
    println!("Using Euclidean distance_use_case calculation");
    // Convert degrees to radians
    let lat1_rad = deg_to_rad(origin_lat);

    // Calculate x and y displacements
    let delta_x = distance * bearing.to_radians().cos();
    let delta_y = distance * bearing.to_radians().sin();

    // Adjust for latitude (meters per degree)
    let meters_per_degree_lat = 111_132.954
        - 559.822 * (2.0 * lat1_rad).cos()
        + 1.175 * (4.0 * lat1_rad).cos();

    let meters_per_degree_lon = 111_412.84 * lat1_rad.cos()
        - 93.5 * (3.0 * lat1_rad).cos()
        + 0.118 * (5.0 * lat1_rad).cos();

    // Convert back to degrees
    let delta_lat_deg = delta_y / meters_per_degree_lat;
    let delta_lon_deg = delta_x / meters_per_degree_lon;

    (origin_lat + delta_lat_deg, origin_long + delta_lon_deg)
}

fn calculate_distance_long(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64
) -> (f64, f64) {
    println!("Using Haversine formula combined with spherical trigonometry for distance_use_case calculation");
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

    (lat2, lon2)
}

fn validate_inputs(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64
) -> Result<(), Error> {
    if origin_lat < -90.0 || origin_lat > 90.0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Latitude must be between -90 and 90 degrees"));
    }

    if origin_long < -180.0 || origin_long > 180.0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Longitude must be between -180 and 180 degrees"));
    }

    if distance < 0.0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Distance must be non-negative"));
    }

    if bearing < 0.0 || bearing >= 360.0 {
        return Err(Error::new(ErrorKind::InvalidInput, "Bearing must be between 0 and 360 degrees"));
    }

    let allowed_units = ["m", "km", "mi"]; // Define the supported units
    if !allowed_units.contains(&unit) {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid unit of measurement"));
    }

    Ok(())
}

fn use_short_distance(
    distance: f64,
    unit: &str
) -> Result<bool, Error>{
    /*
    we accept:
        m = meters
        km = kilometers
        mi = miles
     */

    

    Ok(true)
}

pub fn logic_flow_one(
    origin_lat: f64,
    origin_long: f64,
    distance: f64,
    unit: &str,
    bearing: f64,
) -> Result<PointToPointDistanceResponse, Error> {

    validate_inputs(origin_lat, origin_long, distance, unit, bearing)?;



    // leaving in some debugging stuff to get my output right
    //println!("Origin (Decimal): {:.6}, {:.6}", origin_long, origin_lat);

    // Calculate short and long distance_use_case
    let (target_lat_short, target_lon_short) = calculate_distance_short(origin_lat, origin_long, distance, unit, bearing);
    //println!("Short Distance response");
    //println!("Target (Decimal): {:.6}, {:.6}", target_lon_short, target_lat_short);

    let (target_lat_long, target_lon_long) = calculate_distance_long(origin_lat, origin_long, distance, unit, bearing);
    //println!("Long Distance response");
    //println!("Target (Decimal): {:.6}, {:.6}", target_lon_long, target_lat_long);


    // Convert to DMS format
    let formatted_dms = return_dms_from_lat_long(target_lat_short, target_lon_short);
    //println!("Formatted");
    //println!("{}", formatted_dms);

    // Return values
    //(target_lat_short, target_lon_short, target_lat_long, target_lon_long, formatted_dms)

    Ok(PointToPointDistanceResponse{
        target_lat_short,
        target_lon_short,
        target_lat_long,
        target_lon_long,
        message: formatted_dms
    })
}
