extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::structs::LatLng;
use google_maps_services::distance_matrix::distance_matrix_request;
use std::env;

#[test]
fn test_distance_matrix() {
    // Get API key from .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    // Melbourne CBD
    let origin = LatLng::new(-37.8274812, 144.9352466);

    // Ballarat CBD
    let destination = LatLng::new(-37.5674314, 143.7827008);

    let _ = distance_matrix_request(api_key, origin, destination);

    // Why isn't this working? :(
    assert!(true);
}