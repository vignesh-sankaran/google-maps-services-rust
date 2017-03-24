#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    pub origins: String,
    pub destinations: String,
    pub api_key: String,
}