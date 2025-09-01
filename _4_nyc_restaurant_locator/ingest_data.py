from typing import *
import pandas as pd
import psycopg2
import json

df = pd.read_csv("./data/nyc_restaurants_gmaps.csv")

table="nyc_restaurants"

def clean_detailed_ratings(detailed_ratings:str):
    # dr = detailed_ratings[1..(len(detailed_ratings-1))]
    # dr=detailed_ratings.removeprefix('{')
    # dr=dr.removesuffix('}')
    return detailed_ratings.replace("\'","\"")

try:
    conn = psycopg2.connect(
            database="restaurant_db",
            user="postgis",
            password="postgis123",
            host="localhost",
            port="5432"
        )
    cur = conn.cursor()
    # Truncate table
    cur.execute(f"TRUNCATE table {table}")
    conn.commit()
    
    for index, row in df.iterrows():
        url = str(row['URL'])
        name = str(row['Name']).replace("\'","\"")
        rating = float(row['Rating']) if not pd.isna(row['Rating']) else 0.0
        rating_count = int(row['Rating Count']) if not pd.isna(row['Rating Count']) else 0
        detailed_ratings = clean_detailed_ratings(row['Detailed Ratings'])
        price_cat = int(row['Price Category']) if not pd.isna(row['Price Category']) else 0
        address = str(row['Address'])
        lat = float(row['Lat']) if not pd.isna(row['Lat']) else 0.0
        long = float(row['Lon']) if not pd.isna(row['Lon']) else 0.0
        zipcode = int(row['ZipCode']) if not pd.isna(row['ZipCode']) else 0
        SQL = f"""
            INSERT INTO {table} (
                url, name, rating, rating_count, detailed_ratings,
                price_category, address, location, zipcode
            )
            VALUES (
                %s, %s, %s, %s, %s,
                %s, %s, ST_GeomFromText(%s, 4326),  %s
            );
        """

        # Build WKT string for PostGIS point
        point = f"POINT({lat} {long})"

        cur.execute(SQL, (
            url, name, rating, rating_count, detailed_ratings,
            price_cat, address, point, zipcode
        ))
    conn.commit()
except psycopg2.Error as e:
    print(f"Error connecting to PostgreSQL: {e}")