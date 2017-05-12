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

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitMode {
    Bus,
    Subway,
    Train,
    Tram,
    Rail,
}