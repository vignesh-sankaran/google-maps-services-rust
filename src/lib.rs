extern crate hyper;
extern crate serde_urlencoded;
#[macro_use]
extern crate serde_derive;

pub mod distance_matrix;
pub mod structs;

const BASE_URL: &'static str = "https://maps.googleapis.com/maps/api/";

struct Client {
    pub url: String,
    pub api_key: String,
}