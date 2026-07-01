# $ uvicorn main:app --host 0.0.0.0 --port 56732 --reload
# $ python3 -m uvicorn main:app --host 0.0.0.0 --port 56732 --reload

from fastapi import FastAPI
from fastapi.responses import JSONResponse
from database import connect, disconnect
from routes.user_ops import router as user_router
from routes.devices_ops import router as device_router
from contextlib import asynccontextmanager
import logging

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
    return JSONResponse({"message":"I am user-device registeration server."}, status_code=200)

app.include_router(user_router)
app.include_router(device_router)