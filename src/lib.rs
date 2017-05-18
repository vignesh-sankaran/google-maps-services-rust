//! A Rust bindings library to Google Maps Web Services.
//! 
//! The purpose of this crate is to provide an idomatic Rust bindings library
//! for Google Maps Web Services. It is a work in progress, and there will be
//! regular breaking changes prior to 0.1.0. 
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_urlencoded;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Distance Matrix request and response
pub mod distance_matrix;
/// Common structs related to making a request
pub mod request_structs;
/// Common library wide types
pub mod types;

const BASE_URL: &'static str = "https://maps.googleapis.com/maps/api/";
