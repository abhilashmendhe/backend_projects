import { createRestaurantCard } from "./create_restaurant_card";

export function pinFetchedFestaurants(map, markersLayer, restaurants, leftPane) {

    for (let rest of restaurants) {
        createMarker(map, markersLayer, rest);
        let child = createRestaurantCard(rest);
        leftPane.appendChild(child);
    }
}

function createMarker(map, markersLayer, restaurant) {

    const price_category = restaurant.price_category;
    
    const long = restaurant.location.lat;
    const lat = restaurant.location.lon;
    const coordinates = [lat,long];

    const icon = L.divIcon({
      iconAnchor: [0, 24],
      labelAnchor: [-6, 0],
      popupAnchor: [0, -36],
    })

    if (price_category === 1) {
        icon.options.html= '<div class="bg-[#44ce1b] w-8 h-8 block relative rounded-[3rem_3rem_0_3rem] rotate-45 border border-black"/>';
    } else if (price_category === 2) {
        icon.options.html= '<div class="bg-[#bbdb44] w-8 h-8 block relative rounded-[3rem_3rem_0_3rem] rotate-45 border border-black"/>';
    } else if (price_category === 3) {
        icon.options.html= '<div class="bg-[#f7e379] w-8 h-8 block relative rounded-[3rem_3rem_0_3rem] rotate-45 border border-black"/>';
    } else if (price_category === 4) {
        icon.options.html= '<div class="bg-[#f2a134] w-8 h-8 block relative rounded-[3rem_3rem_0_3rem] rotate-45 border border-black"/>';
    } else {
        icon.options.html= '<div class="bg-[#e51f1f] w-8 h-8 block relative rounded-[3rem_3rem_0_3rem] rotate-45 border border-black"/>';
    }

    let marker = L.marker(coordinates,{icon: icon}).addTo(markersLayer);
    marker.bindPopup(`<h1>${restaurant.name}</h1>`);
}


