extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::distance_matrix::DistanceMatrixRequest;
use google_maps_services::distance_matrix::DistanceMatrixRequestBuilder;
use google_maps_services::types::TravelMode;
use google_maps_services::types::TransitMode;
use std::env;

#[test]
fn address_minimum() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                    .create();

    let result = request.send();

    assert_eq!(result.status, "OK");
}

#[test]
fn address_travel_mode() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let mut request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Transit)
                        .create();

    let result = request.send();

    assert_eq!(result.status, "OK");
}

#[test]
fn address_transit_mode_success() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let mut request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Transit)
                        .set_transit_mode(TransitMode::Rail).unwrap()
                        .create();

    let result = request.send();

    assert_eq!(result.status, "OK");
}

#[test]
#[should_panic]
fn address_transit_mode_fail() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let mut request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Transit)
                        .set_transit_mode(TransitMode::Rail).unwrap()
                        .create();
}
