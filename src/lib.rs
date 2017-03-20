extern crate hyper;
extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod distance_matrix;
pub mod structs;

const BASE_URL: &'static str = "https://maps.googleapis.com/maps/api/";