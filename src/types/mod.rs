#[derive(Serialize)]
pub enum TravelMode {
    #[serde(rename = "driving")]
    Driving,
    #[serde(rename = "walking")]
    Walking,
    #[serde(rename = "bicycling")]
    Bicycling,
    #[serde(rename = "transit")]
    Transit,
}
