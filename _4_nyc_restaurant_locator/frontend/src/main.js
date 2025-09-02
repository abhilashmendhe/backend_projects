
/**
 * fetch request for api
 http://localhost:3000/v1/restaurants?lat={}&long={}&radius={}
e.g. http://localhost:3000/v1/restaurants?lat=40.683282238369586&long=-74.0004668922639&radius=1999

 */

import { pinRestaurants } from "./fetch_restaurants";
import { initMap } from "./load_map";

// document.getElementById('map').style.cursor = 'pointer'
// 1. load map
var [map,markersLayer] = initMap();  

let leftPane = document.getElementById("left-pane");

// console.log(map, markersLayer);
function onMapClick(e) {
    // e.latlng contains the latitude and longitude of the clicked location
    console.log("You clicked the map at: " + e.latlng);
    // You can also display this information in a popup
    L.popup()
        .setLatLng(e.latlng)
        .setContent("You clicked the map at " + e.latlng.toString())
        .openOn(map);
}

map.on('dblclick', (e)=>{
  markersLayer.clearLayers();
  leftPane.replaceChildren();
  pinRestaurants(e, map, markersLayer, leftPane)
});

