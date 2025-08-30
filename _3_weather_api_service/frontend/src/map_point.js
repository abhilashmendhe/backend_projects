export function mapPoint(map, weather_info) {

    let inp_city = weather_info.address;
    let lat = weather_info.latitude;
    let long = weather_info.longitude;
    map.setView([lat, long], 13);

    const currentConditions = weather_info.currentConditions;

    let w_data = `
        <p>Location: ${inp_city}</p>
        <p>Coords: ${lat},${long}</p>
        <p>Description: ${weather_info.description}</p>
        <p>Temperature: ${currentConditions.temp}</p>
        <p>Feels like: ${currentConditions.feelslike}</p>
        <p>UV Index: ${currentConditions.uvindex}</p>
    `
    L.marker([lat, long]).addTo(map)
    //   .bindPopup(`Weather info for location: ${inp_city}`)
        .bindPopup(w_data)
        .openPopup();
}