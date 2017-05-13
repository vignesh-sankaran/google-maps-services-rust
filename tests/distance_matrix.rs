extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::distance_matrix::DistanceMatrixRequest;
use google_maps_services::types::TravelMode;
use google_maps_services::types::TransitMode;
use std::env;

#[test]
fn address_minimum() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let request = DistanceMatrixRequest::new(origin, destination, api_key);

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

    let mut request = DistanceMatrixRequest::new(origin, destination, api_key);
    // Is there a better way to do this? This'd be a hack for someone using this...
    {
        request.set_travel_mode(travel_mode);
    }

    let result = request.send();

    assert_eq!(result.status, "OK");
}

#[test]
fn address_transit_mode_success() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let mut request = DistanceMatrixRequest::new(origin, destination, api_key);
    {
        request.set_travel_mode(TravelMode::Transit);
    }

    request.set_transit_mode(TransitMode::Rail).unwrap();

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

    let mut request = DistanceMatrixRequest::new(origin, destination, api_key);
    {
        request.set_travel_mode(TravelMode::Bicycling);
    }

    let _ = request.set_transit_mode(TransitMode::Rail).unwrap();
}
