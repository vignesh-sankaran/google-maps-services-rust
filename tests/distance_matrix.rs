extern crate google_maps_services;
extern crate dotenv;

use dotenv::dotenv;
use google_maps_services::distance_matrix::DistanceMatrixRequestBuilder;
use google_maps_services::distance_matrix::status::TopLevelStatus;
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

    assert_eq!(result.status, TopLevelStatus::Ok);
    // TODO: Insert assertions against the other response fields too
}

#[test]
fn address_travel_mode() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Transit)
                        .create();

    let result = request.send();

    assert_eq!(result.status, TopLevelStatus::Ok);
}

#[test]
fn address_transit_mode_success() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    let request = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Transit)
                        .set_transit_mode(TransitMode::Rail)
                        .unwrap()
                        .create();

    let result = request.send();

    assert_eq!(result.status, TopLevelStatus::Ok);
}

#[test]
fn address_transit_mode_fail() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set in the .env file");

    let origin = "Melbourne GPO".to_string();
    let destination = "Ballarat CBD".to_string();

    /* 
        TODO: See if there's another way to check if an error has occurred and
        throw a panic when the expected error occurs
    */
    let builder = DistanceMatrixRequestBuilder::new(origin, destination, api_key)
                        .set_travel_mode(TravelMode::Bicycling)
                        .set_transit_mode(TransitMode::Rail);

    match builder {
        Err(e) => {
            assert_eq!(e, google_maps_services::distance_matrix::error::IncompatibleTravelModeError);
        },
        Ok(_) => {
            panic!("There should be an error here");
        },
    }
}
