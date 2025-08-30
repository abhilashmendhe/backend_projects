export async function fetchWeather(url) {
  try {
    const data = await fetch(url); // Wait for the promise to resolve and assign the result
    const weatherData = await data.json();
    return weatherData;
  } catch (error) {
    console.error("Failed to fetch the data", error);
  }
}
