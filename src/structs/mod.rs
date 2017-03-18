use std::fmt;

pub struct LatLng {
    pub lat: i32,
    pub lng: i32,
}

impl LatLng {
    fn new(lat: i32, lng: i32) -> LatLng {
        LatLng {
            lat: lat,
            lng: lng,
        }
    }
}

/// Output a latlong into a single comma-delimited string
impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.lat, self.lng)
    }
}