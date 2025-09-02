
export function initMap() {
    let coordinates = [40.74082673402984, -74.03986752776116];
    let map = L.map('map', { 
        doubleClickZoom: false 
    }).setView(coordinates, 12);

    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(map);
    const markersLayer = L.layerGroup().addTo(map);
    return [map,markersLayer];
}