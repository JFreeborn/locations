use std::f64::consts::PI;

/// Convert degrees to radians
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Convert radians to degrees
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Converts decimal locations into DMS locations
pub fn decimal_to_dms(decimal: f64) -> (i32, i32, f64) {
    let degrees = decimal.trunc() as i32;
    let minutes_decimal = (decimal.abs() - degrees.abs() as f64) * 60.0;
    let minutes = minutes_decimal.trunc() as i32;
    let seconds = (minutes_decimal - minutes as f64) * 60.0;
    (degrees, minutes, seconds)
}
