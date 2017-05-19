// The documentation for these statuses is pretty much a copy from https://developers.google.com/maps/documentation/distance-matrix/usage-limits
/// Status contained at the top level of a `DistanceMatrixResponse` struct
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum TopLevelStatus {
    /// Valid result
    Ok,
    /// An invalid request. These errors should not occur unless there is a bug in this library that allows this error to occur
    InvalidRequest,
    /// Product of origins and destinations exceeds the individual query limit
    /// These limits are defined [here](https://developers.google.com/maps/documentation/distance-matrix/usage-limits API usage limits)
    MaxElementsExceeded,
    /// Too many requests received within the time restrictions specified by the standard or premium service
    OverQueryLimit,
    /// Request denied by Google
    RequestDenied,
    /// Server side error, could succeed again if request tried again
    UnknownError,
}

/// Status contained within a `DistanceMatrixRowObject` struct
#[derive(Deserialize, Debug)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum ElementStatus {
    /// Valid result
    Ok,
    /// Origin or destination could not be geocoded
    NotFound,
    /// No route found between origin and destination
    ZeroResults,
    /// Route is too long and cannot be processed
    MaxRouteLengthExceeded,
}
