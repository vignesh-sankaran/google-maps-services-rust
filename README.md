# google-maps-services-rust
A Rust binding library for Google Maps Web Services. I can't find one that exists so I'm going to start building one myself. Pull requests are more than welcome.

Work in progress and in the pipeline for this project can be found [here](https://tree.taiga.io/project/backend_dev-google-web-services-rust/kanban "Kanban for google-web-services-rust")

## Project goal
The aim is to have a production grade API binding library with integration tests and Travis CI integration.

## Roadmap
In the order of priority, the upcoming planned functionality is:
* Distance Matrix
* Directions

I do plan to build the other web services endpoints in depending on demand. Once the above 2 are done I may put up a survey to establish interest in the future functionality of this library.

## Current status
Distance Matrix is being currently being built. It currently accepts a single origin and destination in a LatLong format, and returns a DistanceMatrixResponse struct. The next things to be worked on, in order are:

* Address insertion for origin and destination
* PlaceID specifier for origin and destination
* Multiple origins and destinations
* Optional fields in a Distance Matrix request