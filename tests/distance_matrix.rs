extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::structs::LatLng;
use google_maps_services::distance_matrix::lat_lng_request;
use google_maps_services::distance_matrix::address_request;
use std::env;

#[test]
fn lat_lng() {
    // Get API key from .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    // Melbourne CBD
    let origin = LatLng::new(-37.8274812, 144.9352466);

    // Ballarat CBD
    let destination = LatLng::new(-37.5674314, 143.7827008);

    let result = lat_lng_request(api_key, origin, destination);

    assert_eq!(result.destination_addresses.len(), 1);
}

#[test]
fn address() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO";
    let destination = "Ballarat CBD";

    let result = address_request(api_key, origin.to_string(), destination.to_string());

    assert_eq!(result.destination_addresses.len(), 1);
}
