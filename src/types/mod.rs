#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TravelMode {
    Driving,
    Walking,
    Bicycling,
    Transit,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitMode {
    Bus,
    Subway,
    Train,
    Tram,
    Rail,
}
