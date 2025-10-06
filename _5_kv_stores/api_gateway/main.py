# Run -> $ uvicorn main:app --reload

from fastapi import FastAPI, Request
from fastapi.responses import RedirectResponse
import httpx
from walstruct import WALKVStore
from pathlib import Path
from models import PutReqKV, GetReqKV

w = WALKVStore(str(Path.cwd().parent))

app = FastAPI()

@app.get("/about")
async def about():
    return {"message":"I am the API gateway for KV store."}

@app.get("/")
async def get_kv(request: Request):

    payload = await request.json()
    async with httpx.AsyncClient() as client:
        response = await client.request(
            url="http://0.0.0.0:58322/api/v1",
            method="GET",
            json=payload
        )
        data = response.json()
    return data 

@app.put("/", status_code=201)
async def put_kv(request: Request):
    
    payload = await request.json()
    prkv = PutReqKV(**payload).model_dump()
    w.write_record(kvs=prkv)
    
    async with httpx.AsyncClient() as client:
        response = await client.request(
            url="http://0.0.0.0:58322/api/v1",
            method="PUT",
            json=payload
        )
    
    
    