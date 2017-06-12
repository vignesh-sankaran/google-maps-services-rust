use std::error::Error;
use std::fmt;

//// An error that describes a situation where `TravelMode` is not set to
/// `Transit` and the `TransitMode` is attempted to be accessed. This is
/// not supported by Google Maps Web Services.
#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct ResponseError;

#[derive(Debug)]
enum ResponseErrorKind {
    TopLevelError(TopLevelErrorKind),
    ElementErrorKind(ElementErrorKind),
}

#[derive(Debug)]
enum TopLevelErrorKind {
    InvalidRequest,
    MaxElementsExceeded,
    OverQueryLimit,
    RequestDenied,
    UnknownError,
}

impl TopLevelErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            TopLevelErrorKind::InvalidRequest => "Invalid request",
            TopLevelErrorKind::MaxElementsExceeded => "Max elements exceeded",
            TopLevelErrorKind::OverQueryLimit => "Over query limit",
            TopLevelErrorKind::RequestDenied => "Request denied",
            TopLevelErrorKind::UnknownError => "Unknown error",
        }
    }
}

#[derive(Debug)]
enum ElementErrorKind {
    NotFound,
    ZeroResults,
    MaxRouteLengthExceeded,
}

impl ElementErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ElementErrorKind::NotFound => "Not found",
            ElementErrorKind::ZeroResults => "Zero results",
            ElementErrorKind::MaxRouteLengthExceeded => "Max route length exceeded",
        }
    }
}
