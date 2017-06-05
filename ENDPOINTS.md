# Endpoint query support
This is a list of endpoints contained within Google Maps Web Services and their request parameters. The list will be further added to and updated over time, and also plan to add a list of return parameters for each endpoint.

## Distance Matrix
### Mandatory fields
[ ] Origin
    [ ] Address
    [x] Lat Long
    [ ] Place ID (premium account only)
[ ] Destination
    [ ] Address
    [x] Lat Long
    [ ] Place ID (premium account only)
[x] API Key

Multiple values for Origin and Destination currently not supported

### Optional fields
[x] Travel Mode
[ ] Language
[ ] Avoid
[ ] Units
[ ] Arrival time
[ ] Departure time
[ ] Traffic model
[x] Transit mode
[ ] Transit routing preference