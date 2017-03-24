mod request;
mod response;

use super::structs::LatLng;
use self::request::DistanceMatrixRequest;
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

pub fn distance_matrix_request(api_key: String, origin: LatLng, destination: LatLng) -> DistanceMatrixResponse {
    let request = DistanceMatrixRequest {
        origins: origin.to_string(),
        destinations: destination.to_string(),
        api_key: api_key,
    };

    // Safe to unwrap since external code that doesn't pass in any values will not compile
    let request_params_encoded = serde_urlencoded::to_string(&request).unwrap();
    println!("{}", request.origins);
    println!("{}", request_params_encoded);
    let request_url = super::BASE_URL.to_string() + URL_EXTENSION + "?" + request_params_encoded.as_str();
    println!("{}", request_url);
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
