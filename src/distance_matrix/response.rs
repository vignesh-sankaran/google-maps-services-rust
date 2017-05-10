use std::time::Duration;
use super::duration_seconds;

/* All structs here have to be public because
the Rust compiler will complain that DistanceMatrixResponse
is leaking a private type.  */

#[derive(Deserialize)]
pub struct DistanceMatrixDistanceUnit {
    // Distance in metres
    pub value: f32,
    pub text: String,
}

#[derive(Deserialize)]
pub struct DistanceMatrixDurationUnit {
    // Duration in seconds
    #[serde(with = "duration_seconds")]
    pub value: Duration,
    pub text: String,
}

#[derive(Deserialize)]
pub struct DistanceMatrixRowObject {
    distance: DistanceMatrixDistanceUnit,
    duration: DistanceMatrixDurationUnit,
    status: String,
}

#[derive(Deserialize)]
pub struct Row {
    pub elements: Vec<DistanceMatrixRowObject>,
}

#[derive(Deserialize)]
pub struct DistanceMatrixResponse {
    pub origin_addresses: Vec<String>,
    pub destination_addresses: Vec<String>,
    pub rows: Vec<Row>,
    pub status: String,
}
