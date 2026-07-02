# $ uvicorn main:app --host 0.0.0.0 --port 8080 --reload

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from contextlib import asynccontextmanager
from routes.users.users import router as user_router
from routes.devices.devices import router as device_router
from config import settings

@asynccontextmanager
async def server_lifespan(app: FastAPI):
    print("🚀 Starting up....")
    yield
    print("🛑 Shutting down...")

app = FastAPI(lifespan=server_lifespan)

@app.get("/about")
async def about():
    return {"message":"I am API gateway for notification systems/services"}

app.include_router(user_router)
app.include_router(device_router)