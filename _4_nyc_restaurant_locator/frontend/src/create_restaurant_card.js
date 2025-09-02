export function createRestaurantCard(restaurant) {
  const card = document.createElement("div");
  card.classList.add(
    "h-48",              // fixed height (adjust as needed)
    "bg-white",
    "rounded-xl",
    "border",
    "border-gray-300",
    "p-4",
    "flex",
    "flex-col",
    "justify-between",
    "shadow-sm"
  );

  // Restaurant name
  const name = document.createElement("h2");
  name.classList.add("text-lg", "font-semibold", "truncate");
  name.textContent = restaurant.name;

  // Address
  const address = document.createElement("p");
  address.classList.add("text-sm", "text-gray-600", "truncate");
  address.textContent = restaurant.address;

  // Rating
  const rating = document.createElement("p");
  rating.classList.add("text-sm", "text-yellow-600", "font-medium");
  rating.textContent = `⭐ ${restaurant.rating} (${restaurant.rating_count} reviews)`;

  // Price category
  const price = document.createElement("p");
  price.classList.add("text-sm", "text-gray-700");
  price.textContent = `Price Category: ${restaurant.price_category}`;

  // Link
  const link = document.createElement("a");
  link.classList.add("text-blue-500", "hover:underline", "text-sm");
  link.href = restaurant.url;
  link.target = "_blank";
  link.textContent = "View on Google Maps";

  // Append to card
  card.appendChild(name);
  card.appendChild(address);
  card.appendChild(rating);
  card.appendChild(price);
  card.appendChild(link);

  return card;
}