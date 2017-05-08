use types::TravelMode;

#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    pub origins: String,
    pub destinations: String,
    pub api_key: String,
    pub travel_mode: Option<TravelMode>,
}

pub struct DistanceMatrixRequestBuilder {
    pub origins: String,
    pub destinations: String,
    pub api_key: String,
    pub travel_mode: Option<TravelMode>,
}

impl DistanceMatrixRequestBuilder {
    pub fn new() -> DistanceMatrixRequestBuilder {
        DistanceMatrixRequestBuilder {
            origins: "".to_string(),
            destinations: "".to_string(),
            api_key: "".to_string(),
            travel_mode: None,
        }
    }

    pub fn origins(&mut self, origins: String) -> &mut DistanceMatrixRequestBuilder {
        self.origins = origins;
        self
    }

    pub fn destinations(&mut self, destinations: String) -> &mut DistanceMatrixRequestBuilder {
        self.destinations = destinations;
        self
    }

    pub fn api_key(&mut self, api_key: String) -> &mut DistanceMatrixRequestBuilder {
        self.api_key = api_key;
        self
    }

    pub fn travel_mode(&mut self, travel_mode: Option<TravelMode>) -> &mut DistanceMatrixRequestBuilder {
        self.travel_mode = travel_mode;
        self
    }

    pub fn create(&self) -> DistanceMatrixRequest {
        DistanceMatrixRequest {
            origins: self.origins,
            destinations: self.destinations,
            api_key: self.api_key,
            travel_mode: self.travel_mode,
        }
    }
}