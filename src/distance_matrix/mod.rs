/// Errors related to `distance_matrix`
pub mod error;
/// Status of a `DistanceMatrixResponse`
pub mod status;
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

/// A builder to construct a `DistanceMatrixRequest`
///
/// Usage: Use `new()` to pass in the basic parameters, pass in optional parameters
/// using the apporpriate setter function, then create a `DistanceMatrixRequest` from
/// the given parameters with `create()`
pub struct DistanceMatrixRequestBuilder {
    origins: String,
    destinations: String,
    api_key: String,
    travel_mode: Option<TravelMode>,
    transit_mode: Option<TransitMode>,
}

impl DistanceMatrixRequestBuilder {
    /// Creates a new distance matrix request from the mandatory fields
    pub fn new(origins: String, destinations: String, api_key: String) -> DistanceMatrixRequestBuilder {
        DistanceMatrixRequestBuilder {
            origins: origins,
            destinations: destinations,
            api_key: api_key,
            travel_mode: None,
            transit_mode: None,
        }
    }

    /// Set the travel mode. Default mode is driving
    pub fn set_travel_mode(mut self, travel_mode: TravelMode) -> DistanceMatrixRequestBuilder{
        self.travel_mode = Some(travel_mode);
        self
    }

    /// Set the transit mode
    /// # Errors
    /// This function will return an `IncompatibleTravelModeError` if the `TravelMode` is
    /// other than `Transit` or set to `None`
    pub fn set_transit_mode(mut self, transit_mode: TransitMode) -> Result<DistanceMatrixRequestBuilder, error::IncompatibleTravelModeError> {
        match self.travel_mode {
            Some(TravelMode::Transit) => {
                self.transit_mode = Some(transit_mode);
                Ok(self)
            },
            Some(_) => Err(error::IncompatibleTravelModeError),
            None => Err(error::IncompatibleTravelModeError),
        }
    }

    /// Returns a `DistanceMatrixRequest` with the values specified in the builder
    pub fn create(self) -> DistanceMatrixRequest {
        DistanceMatrixRequest {
            origins: self.origins,
            destinations: self.destinations,
            api_key: self.api_key,
            travel_mode: self.travel_mode,
            transit_mode: self.transit_mode,
        }
    }
}

 /// A struct containing the parameters to make a Distance Matrix request
#[derive(Serialize)]
pub struct DistanceMatrixRequest {
    origins: String,
    destinations: String,
    api_key: String,
    travel_mode: Option<TravelMode>,
    transit_mode: Option<TransitMode>,
}

impl DistanceMatrixRequest {
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
