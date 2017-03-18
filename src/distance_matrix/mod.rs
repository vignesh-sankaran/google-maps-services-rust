mod request;

use super::Client;
use super::structs::LatLng;
use self::request::DistanceMatrixRequest;
use hyper;
use serde_urlencoded;

// Default use JSON
const URL_EXTENSION: &'static str = "distancematrix/json?";

pub fn distance_matrix_request(origin: LatLng, destination: LatLng) {
    let request = DistanceMatrixRequest {
        origin: origin.to_string(),
        destination: destination.to_string(),
    };

    let request_url_encoded = serde_urlencoded::to_string(&request);

}

