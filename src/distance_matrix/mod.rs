use super::request_structs::LatLng;
use super::types::TravelMode;
use self::response::DistanceMatrixResponse;
use std::io::Read;
use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_urlencoded;
use serde_json;

mod response;
mod duration_seconds;

// Default use JSON
const URL_EXTENSION: &'static str = "distancematrix/json";

/* Note: If a struct value is of None type, it is not appended to
 the generated URL */
#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    origins: String,
    destinations: String,
    api_key: String,
    travel_mode: Option<TravelMode>,
}

impl DistanceMatrixRequest {
    pub fn new(origins: String, destinations: String, api_key: String) -> DistanceMatrixRequest {
        DistanceMatrixRequest {
            origins: origins,
            destinations: destinations,
            api_key: api_key,
            travel_mode: None,
        }
    }

    pub fn set_travel_mode(&mut self, travel_mode: TravelMode) {
        self.travel_mode = Some(travel_mode);
    }

    // This is probably not thread safe but we'll worry about that later
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
