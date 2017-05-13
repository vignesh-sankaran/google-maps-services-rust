use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IncompatibleTravelModeError;

impl Error for IncompatibleTravelModeError {
    fn description(&self) -> &str {
        "A TravelMode must be set to Transit for a TransitMode to be settable"
    }
}

impl fmt::Display for IncompatibleTravelModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
