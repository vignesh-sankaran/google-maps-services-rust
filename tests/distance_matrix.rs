extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::request_structs::LatLng;
use google_maps_services::types::TravelMode;
use std::env;

#[test]
fn address_minimum() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let request = request::new(origin, destination, api_key);

    let result = request.send();

    assert_eq!(result.status, "OK");
}

#[test]
fn address_travel_mode() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();
    let travel_mode = TravelMode::Transit;

    let request = request::new(origin, destination, api_key);
    request.set_travel_mode(travel_mode);

    let result = request.send();

    assert_eq!(result.status, "OK");
}