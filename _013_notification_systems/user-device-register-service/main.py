# $ uvicorn main:app --host 0.0.0.0 --port 56732 --reload

from fastapi import FastAPI
from fastapi.responses import JSONResponse
from database import connect, disconnect, get_connection
from contextlib import asynccontextmanager

@asynccontextmanager
async def server_lifespan(app: FastAPI):
    print("🚀 Starting up user device register server....")
    await connect()
    yield
    await disconnect()
    print("🛑 Shutting down user device reigster server...")

app = FastAPI(lifespan=server_lifespan)

@app.get("/about")
async def about():
    data = {"message":"I am user-device registeration server."}
    return JSONResponse(data, status_code=200)