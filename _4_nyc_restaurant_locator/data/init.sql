CREATE EXTENSION IF NOT EXISTS dblink;

DO
$$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_database WHERE datname = 'restaurant_db'
   ) THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE restaurant_db');
   END IF;
END
$$;

\connect restaurant_db;

CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE IF NOT EXISTS nyc_restaurants(
    id SERIAL PRIMARY KEY,
    url TEXT,
    name TEXT,
    rating DOUBLE PRECISION,
    rating_count INTEGER,
    detailed_ratings JSONB,
    price_category INTEGER,
    address TEXT,
    location GEOMETRY(Point, 4326),
    zipcode INTEGER
);
