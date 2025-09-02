import { pinFetchedFestaurants } from "./pin_fetched_restauratns";

export async function pinRestaurants(e, map, markersLayer, leftPane) {

    const { lat,lng } = e.latlng;
    // console.log(`You clicked on ${lat} ${lng}`);
    // console.log(map);
    // console.log(markersLayer);

    let radius = Number(prompt("Enter radius in meters."));
    if (typeof radius === 'number' && radius > 0) {

        // 1. fetch restaurants
        let restaurants = await fetchRestaurants(lat,lng,radius);

        pinFetchedFestaurants(map, markersLayer, restaurants.data, leftPane);

    } else {
        alert("Not a number. Please enter a +ve value")
    }
}

async function fetchRestaurants(lat, long, radius) {

    const url = `http://localhost:3000/v1/restaurants?lat=${lat}&long=${long}&radius=${radius}`;

    const result = await fetch(url);
    const restaurants = await result.json();
    
    return restaurants;
}