Fetching muni data:
https://gist.github.com/grantland/7cf4097dd9cdf0dfed14


Fetching a map:

https://api.mapbox.com/styles/v1/mapbox/streets-v10/static/-122.480243,37.759366,12.8,0,0/400x240?access_token=pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A


curl "https://api.mapbox.com/styles/v1/mapbox/streets-v10/static/url-https%3A%2F%2Fwww.mapbox.com%2Fimg%2Frocket.png(-76.9,38.9)/-76.9,38.9,15/1000x1000?access_token=pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A"


# to geojson:
cat 7-data.json | jq '[.route.stop[] | { type: "Feature", properties: {}, geometry: { type: "Point", coordinates: [.lon, .lat | tonumber] } }]' > Downloads/points.json


# getting tiles:
Mapbox static tiles: https://www.mapbox.com/api-documentation/#retrieve-a-static-map-from-a-style
Coordinates to tile number: https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Lon..2Flat._to_tile_numbers_2
https://api.mapbox.com/styles/v1/mapbox/streets-v10/tiles/12/654/1583?access_token=pk.eyJ1IjoiYnNzZGsiLCJhIjoiY2prYW42NWFlMjZkNzNra3lnYnB6djRscCJ9.KEJKmTjzzjtKVMyxS_Y93A