use std::time::Duration;

/* All structs here have to be public because
the Rust compiler will complain that DistanceMatrixResponse
is leaking a private type.  */

#[derive(Deserialize)]
pub struct DistanceMatrixDistanceUnit {
    // Distance in metres
    pub distance: f32,
    pub text: String,
}

#[derive(Deserialize)]
pub struct DistanceMatrixDurationUnit {
    // Duration in seconds
    pub value: Duration,
    pub text: String,
}

#[derive(Deserialize)]
pub struct DistanceMatrixRowObject {
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