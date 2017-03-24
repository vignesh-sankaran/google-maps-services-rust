use std::fmt;

pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> LatLng {
        LatLng {
            lat: lat,
            lng: lng,
        }
    }
}

/// Output a latlong into a single comma-delimited string
impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.lat.to_string(), self.lng.to_string())
    }
}