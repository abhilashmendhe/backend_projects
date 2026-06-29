# $ uvicorn main:app --host 0.0.0.0 --port 8080 --reload


from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from contextlib import asynccontextmanager
import httpx
import os

PORT = int(os.environ.get("PORT", "8080"))
GATEWAY_URL = os.environ.get("GATEWAY_URL", "http://localhost:9000")

@asynccontextmanager
async def server_lifespan(app: FastAPI):
    print("🚀 Starting up....")
    yield
    print("🛑 Shutting down...")

app = FastAPI(lifespan=server_lifespan)

@app.get("/about")
async def about():
    return {"message":"I am API gateway for notification systems/services"}