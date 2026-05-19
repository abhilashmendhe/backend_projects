#### To run
# $ pipx inject chromadb fastapi uvicorn
# $ clear; uvicorn main:app --host 0.0.0.0 --port 8000 --reload

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
import httpx 
import os 
from contextlib import asynccontextmanager
from routes.users.user_routes import router as user_router
from routes.tinyurl.tinyurl_routes import router as tinyurl_router

@asynccontextmanager
async def server_info_lifespan(app: FastAPI):
    print("🚀 Starting up....")
    yield
    print("🛑 Shutting down...")

app = FastAPI(lifespan=server_info_lifespan)

@app.get("/about")
async def about():
    return {"message":"I am load balancer for tinyurl"}

app.include_router(user_router)
app.include_router(tinyurl_router)