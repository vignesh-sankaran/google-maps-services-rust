# Endpoint query support
This is a list of endpoints contained within Google Maps Web Services, and what is currently implemented and yet to be supported. 

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