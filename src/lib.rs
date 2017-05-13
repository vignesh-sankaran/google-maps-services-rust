//! A Rust bindings library to Google Maps Web Services.
//! 
//! The purpose of this crate is to provide a idomatic Rust bindings library
//! for Google Maps Web Services, since there doesn't appear to be on that exists :)
//! Feel free to put in feature requests and pull requests if you see something 
//! that can be added or improved.
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_urlencoded;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod distance_matrix;
pub mod request_structs;
pub mod types;

const BASE_URL: &'static str = "https://maps.googleapis.com/maps/api/";
