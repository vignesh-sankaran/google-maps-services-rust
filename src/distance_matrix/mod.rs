use super::request_structs::LatLng;
use super::types::TravelMode;
use self::request::DistanceMatrixRequest;
use self::response::DistanceMatrixResponse;
use std::io::Read;
use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_urlencoded;
use serde_json;

mod request;
mod response;
/*
    Using this mod file means this is the only place I can
    declare all other modules needed i.e. I can't nest modules
*/
mod duration_seconds;

// Default use JSON
const URL_EXTENSION: &'static str = "distancematrix/json";

// This is probably not thread safe but we'll worry about that later
fn request(request_data: DistanceMatrixRequest) -> DistanceMatrixResponse {
    // Safe to unwrap since external code that doesn't pass in any values will not compile
    let request_params_encoded = serde_urlencoded::to_string(&request_data).unwrap();
    let request_url = super::BASE_URL.to_string() + URL_EXTENSION + "?" + request_params_encoded.as_str();
    let processed_url = hyper::Url::parse(&request_url).unwrap();
    
    let ssl = NativeTlsClient::new().unwrap();
    let ssl_connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(ssl_connector);

    let mut response = client.get(processed_url).send().unwrap();
    let mut response_string = String::new();
    {
        response.read_to_string(&mut response_string).unwrap();
    }
    let distance_matrix_response: DistanceMatrixResponse = serde_json::from_str(&response_string).unwrap();

    distance_matrix_response
}

pub fn lat_lng_request(api_key: String, origin: LatLng, destination: LatLng) -> DistanceMatrixResponse {
    let request_data = DistanceMatrixRequest {
        origins: origin.to_string(),
        destinations: destination.to_string(),
        api_key: api_key,
        travel_mode: None,
    };

    request(request_data)
}

pub fn address_request(api_key: String, origin: String, destination: String) -> DistanceMatrixResponse {
    let request_data = DistanceMatrixRequest {
        origins: origin,
        destinations: destination,
        api_key: api_key,
        travel_mode: None,
    };

    request(request_data)
}
