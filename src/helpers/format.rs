use crate::helpers::math::decimal_to_dms;

pub fn return_dms_from_lat_long(latitude: f64, longitude: f64) -> String {
    let (lat_deg, lat_min, lat_sec) = decimal_to_dms(latitude);
    let (lon_deg, lon_min, lon_sec) = decimal_to_dms(longitude);

    let formatted_lat = format_dms(
        lat_deg.abs(),
        lat_min,
        lat_sec,
        if latitude >= 0.0 { "N" } else { "S" },
    );
    let formatted_lon = format_dms(
        lon_deg.abs(),
        lon_min,
        lon_sec,
        if longitude >= 0.0 { "E" } else { "W" },
    );

    format!("{} {}", formatted_lat, formatted_lon)
}

fn format_dms(degrees: i32, minutes: i32, seconds: f64, hemisphere: &str) -> String {
    format!("{}°{}'{:.1}\"{}", degrees, minutes, seconds, hemisphere)
}