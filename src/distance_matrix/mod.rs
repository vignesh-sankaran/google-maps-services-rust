pub mod error;
mod response;
mod duration_seconds;

use super::types::TravelMode;
use super::types::TransitMode;
use self::response::DistanceMatrixResponse;
use std::io::Read;
use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_urlencoded;
use serde_json;

// Default use JSON
const URL_EXTENSION: &'static str = "distancematrix/json";

/* Note: If a struct value is of None type, it is not appended to
 the generated URL */
 /// A struct for creating and sending distance matrix requests
#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    origins: String,
    destinations: String,
    api_key: String,
    travel_mode: Option<TravelMode>,
    transit_mode: Option<TransitMode>,
}

impl DistanceMatrixRequest {
    /// Creates a new distance matrix request from the mandatory fields
    pub fn new(origins: String, destinations: String, api_key: String) -> DistanceMatrixRequest {
        DistanceMatrixRequest {
            origins: origins,
            destinations: destinations,
            api_key: api_key,
            travel_mode: None,
            transit_mode: None,
        }
    }

    /// Set the travel mode. Default mode is driving
    pub fn set_travel_mode(&mut self, travel_mode: TravelMode) {
        self.travel_mode = Some(travel_mode);
    }

    /// Set the transit mode. `TravelMode` must be set to `Transit` for `TransitMode` to be set
    pub fn set_transit_mode(&mut self, transit_mode: TransitMode) -> Result<(), error::IncompatibleTravelModeError> {
        match self.travel_mode {
            Some(TravelMode::Transit) => Ok(self.transit_mode = Some(transit_mode)),
            Some(_) => Err(error::IncompatibleTravelModeError),
            None => Err(error::IncompatibleTravelModeError),
        }
    }

    /// Send a `DistanceMatrixRequest`. Response is currently not case analysed for possible errors
    /// This function is probably also not thread safe
    pub fn send(self) -> DistanceMatrixResponse {
        // Safe to unwrap since external code that doesn't pass in any values will not compile
        let request_params_encoded = serde_urlencoded::to_string(&self).unwrap();
        let request_url = super::BASE_URL.to_string() + URL_EXTENSION + "?" +
                          request_params_encoded.as_str();
        let processed_url = hyper::Url::parse(&request_url).unwrap();

        let ssl = NativeTlsClient::new().unwrap();
        let ssl_connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(ssl_connector);

        let mut response = client.get(processed_url).send().unwrap();
        let mut response_string = String::new();
        {
            response.read_to_string(&mut response_string).unwrap();
        }
        let distance_matrix_response: DistanceMatrixResponse =
            serde_json::from_str(&response_string).unwrap();

        distance_matrix_response
    }
}
