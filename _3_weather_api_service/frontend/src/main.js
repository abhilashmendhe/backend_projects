import { initializeData } from './load_cities.js';
import { fetchWeather } from './fetch_weather_request.js';
import { mapPoint } from './map_point.js';

// 0. set map
var map = L.map('map').setView([51.505, -0.09], 13);

L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
}).addTo(map);


// 1. read all cities-countries from .json file and store in it a varialble
// const citiesData = await initializeData();

// 2. read input from city 
const cityElem = document.querySelector("#city");
const latitudeElem = document.querySelector("#latitude");
const longitudeElem = document.querySelector("#longitude");

// 4.  form handling and fetch GET request
async function fetchWeatherInfo(e) {
  e.preventDefault();
  

  let url = `http://localhost:3000/v1/weather?location=`;

  const inp_city = cityElem.value.trim();
  const inp_latitude = Number(latitudeElem.value.trim());
  const inp_longitude = Number(longitudeElem.value.trim());

  if (!inp_city && (!inp_latitude && !inp_longitude)) {
    alert("Please either enter `city` or (`latitude`,`longitude`)")
  } 

  if (inp_city) {

    url = url + inp_city;
    // console.log(url);
    const weather_info = await fetchWeather(url);
    mapPoint(map, weather_info);

  } else if (inp_latitude && inp_longitude) {
    
    if (typeof inp_latitude === 'number' && typeof inp_longitude === 'number'){
    
      url = url + inp_latitude + "," + inp_longitude;
      // console.log(url);
      const weather_info = await fetchWeather(url);
      mapPoint(map, weather_info);
     
    } else {
    
      alert("Both (`latitude`,`longitude`) should be float values")
    }
  } else {
    
    alert("Please either enter `city` or (`latitude`,`longitude`)")
  }

}

let form = document.querySelector("#formData");
form.addEventListener('submit', fetchWeatherInfo);

document.querySelector("#clear-btn")
  .addEventListener("click",()=>{
    cityElem.value = "";
    latitudeElem.value = "";
    longitudeElem.value = "";
  })