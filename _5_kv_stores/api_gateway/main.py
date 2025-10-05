# Run -> $ uvicorn main:app --reload

from fastapi import FastAPI, Request
from fastapi.responses import RedirectResponse
import httpx

app = FastAPI()

@app.get("/about")
async def about():
    return {"message":"I am the API gateway for KV store."}

@app.get("/")
async def get_kv(request: Request):
    j_data = await request.json()
    async with httpx.AsyncClient() as client:
        response = await client.request(
            url="http://0.0.0.0:58322/api/v1",
            method="GET",
            json=j_data
        )
        data = response.json()
    return data 

@app.put("/")
async def put_kv(request: Request):
    j_data = await request.json()
    async with httpx.AsyncClient() as client:
        response = await client.request(
            url="http://0.0.0.0:58322/api/v1",
            method="PUT",
            json=j_data
        )