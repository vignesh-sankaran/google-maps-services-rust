use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IncompatibleTravelModeError {
    WrongTravelModeError,
    NoTravelModeError,
} 

impl Error for IncompatibleTravelModeError {
    fn description(&self) -> &str {
        match *self {
            ref WrongTravelModeError => "The TravelMode entered cannot have a TransitMode set",
            ref NoTravelModeError => "A TravelMode must be set to Transit for a TransitMode to be settable",
        }
    }
}

impl fmt::Display for IncompatibleTravelModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ref WrongTravelModeError => write!(f, "{}", self.description()),
            ref NoTravelModeError => write!(f, "{}", self.description()),
        }
    }
}
