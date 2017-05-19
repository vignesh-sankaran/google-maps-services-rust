use std::time::Duration;
use super::duration_seconds;
use super::status;

/* All structs here have to be public because
the Rust compiler will complain that DistanceMatrixResponse
is leaking a private type.  */

#[derive(Debug, Deserialize)]
pub struct DistanceMatrixDistanceUnit {
    // Distance in metres
    pub value: f32,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct DistanceMatrixDurationUnit {
    // Duration in seconds
    #[serde(with = "duration_seconds")]
    pub value: Duration,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct DistanceMatrixRowObject {
    pub distance: DistanceMatrixDistanceUnit,
    pub duration: DistanceMatrixDurationUnit,
    pub status: status::ElementStatus,
}

#[derive(Debug, Deserialize)]
pub struct Row {
    pub elements: Vec<DistanceMatrixRowObject>,
}

/// Struct for storing a response for a `DistanceMatrixRequest` struct
#[derive(Debug, Deserialize)]
pub struct DistanceMatrixResponse {
    pub origin_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub rows: Vec<Row>,
    pub status: status::TopLevelStatus,
}
