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
