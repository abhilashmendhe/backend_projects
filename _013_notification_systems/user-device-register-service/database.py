import os
from dotenv import load_dotenv
from psycopg_pool import AsyncConnectionPool

load_dotenv()

DB_URL=os.getenv("DB_URL")

if not DB_URL:
    raise RuntimeError("DB Url not found or set.")

pool = AsyncConnectionPool(DB_URL, open=False)

async def connect():
    await pool.open()
    
async def disconnect():
    await pool.close()
    
async def get_connection():
    async with pool.connection() as conn:
        yield conn 