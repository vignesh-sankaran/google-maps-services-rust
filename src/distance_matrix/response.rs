use serde_derive;
use std::time::Duration;

/* All structs here have to be public because
the Rust compiler will complain that DistanceMatrixResponse
is leaking a private type.  */

#[derive(Deserialize)]
pub struct DistanceMatrixDistanceUnit {
    // Distance in metres
    pub value: String,
    pub text: u32,
}

#[derive(Deserialize)]
pub struct DistanceMatrixDurationUnit {
    // Duration in seconds
    pub value: Duration,
    pub text: String,
}

#[derive(Deserialize)]
pub struct DistanceMatrixRowObject {
    pub distance: DistanceMatrixDistanceUnit,
    pub duration: DistanceMatrixDurationUnit,
}

#[derive(Deserialize)]
pub struct DistanceMatrixResponse {
    pub origin_addresses: Vec<String>,
    pub destination_address: Vec<String>,
    pub rows: Vec<DistanceMatrixRowObject>,
    pub status: String,
}