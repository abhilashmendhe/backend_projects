export async function initializeData() {
  try {
    const data = await fetch('../public/cities.json'); // Wait for the promise to resolve and assign the result
    const citiesData = await data.json();
    return citiesData;
  } catch (error) {
    console.error("Failed to load cities json data:", error);
  }
}
