# Google Maps Web Services - Rust bindings
[![Build Status](https://travis-ci.org/vignesh-sankaran/google-maps-services-rust.svg?branch=master)](https://travis-ci.org/vignesh-sankaran/google-maps-services-rust)
A Rust binding library for Google Maps Web Services. I can't find one that exists so I'm going to start building one myself. Pull requests are more than welcome.

Fair warning to anyone using this crate, the API surface is unstable and will be subject to breaking changes prior to 1.0.0. I will research testing methods to forewarn of breaking changes in upcoming crate versions.

Work in progress and in the pipeline for this project can be found [here](https://tree.taiga.io/project/backend_dev-google-web-services-rust/kanban "Kanban for google-web-services-rust")

## Project goals
The aim is to have a production grade API binding library with integration tests and Travis CI integration. In the long run, I would like to see this officially adopted by Google as one of their client libraries listed [here](https://developers.google.com/maps/documentation/distance-matrix/client-library "Google Maps Web Services official client libraries")

## Roadmap
The first endpoint that will be fully functional is Distance Matrix. Once this is done, I will ask the community to determine what functionality should be built next.

### Pathway to 0.0.1
There are a few things I feel need to be implemented before putting this up on crates.io. Those things are as follows:
- [ ] Documentation with working examples
- [ ] Error handling from API
- [ ] More robust integration testing

## Current status
Distance Matrix is being currently being built. It currently accepts a single origin and destination in a String format, and returns a DistanceMatrixResponse struct. Setting up infrastructure for an initial public 0.0.1 release is currently the priority. After this is done, the next things to be worked on, in order are:

* Optional fields in a Distance Matrix request: currently working on transit mode
* Multiple origins and destinations in Distance Matrix
