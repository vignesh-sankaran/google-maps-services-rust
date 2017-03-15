extern crate hyper;

pub mod distance_matrix;

const BASE_URL: &'static str = "https://maps.googleapis.com/maps/api/";

struct Client {
    url: String,
    api_key: String,
}