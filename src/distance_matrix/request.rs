use serde_derive;

#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    pub origin: String,
    pub destination: String,
    pub api_key: String,
}