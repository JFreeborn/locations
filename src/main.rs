mod use_case;

fn main() {
    // This is going to act as an Input from external data for now.
    // These values will come from a UI into an API - then flow into this code.
    // For now, they are going to be hardcoded for testing and development.
    // There are going to be multiple flows, eventually. For now a start_ will be called in this file.
    start_flow_one();
}

/// take in a single lat/long point
/// take in a distance
/// take in a distance unit
/// return the new lat/long point
fn start_flow_one(){
    // Starting lat/long = 34°53'44.9"N 70°54'48.7"E
    /// right now this does not take in the dms, it takes in the decimal variant.
    /// TODO - update to take in either dms or decimal.
    // starting lat
    let origin_lat = 34.895808;
    // starting long
    let origin_long = 70.913514;
    // distance of the new point from origin
    let distance = 100.0;
    // unit of distance. for now only m = meters is supported
    let unit = "m";
    // The degrees off 0 (north) for the new point
    let bearing = 45.0;

    use_case::flow_one(
        origin_lat,
        origin_long,
        distance,
        unit,
        bearing
    );
}