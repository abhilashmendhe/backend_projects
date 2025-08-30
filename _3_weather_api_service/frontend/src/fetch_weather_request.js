export async function fetchWeather(url) {
  try {
    const data = await fetch(url); // Wait for the promise to resolve and assign the result
    // console.log("Status:",data.status);
    if (data.status===200) {
      const weatherData = await data.json();
      return weatherData;
    } 
    alert("Too many reqests. Wait for 60 seconds.");
  } catch (error) {
    console.error("Failed to fetch the data", error);
  }
}
