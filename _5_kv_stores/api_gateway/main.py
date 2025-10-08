# Run -> $ KV_SERVICE_URL="http://0.0.0.0:58322/api/v1" uvicorn main:app --host 0.0.0.0 --port 8000 --workers 2 --reload

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
import httpx
from walstruct import WALKVStore
from pathlib import Path
from models import PutReqKV
import os 
from contextlib import asynccontextmanager

w = WALKVStore(str(Path.cwd().parent))

KV_SERVICE_URL=os.getenv("KV_SERVICE_URL","http://0.0.0.0:58322/api/v1")


@asynccontextmanager
async def server_info_lifespan(app: FastAPI):
    print("🚀 Starting up....")
    yield
    print("🛑 Shutting down...")

app = FastAPI(lifespan=server_info_lifespan)

@app.get("/about")
async def about():
    return {"message":"I am the API gateway for KV store."}

@app.get("/")
async def get_kv(request: Request):

    payload = await request.json()
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=KV_SERVICE_URL,
                method="GET",
                json=payload
            )
            data = response.json()
        if response.status_code == 200:
            return data  
        data = {"message":"Not found"}
        return JSONResponse(content=data, status_code=404)
    
    except:
        data = {"message":"Internal server error.", "status":"Failed"}
        return JSONResponse(content=data, status_code=500)
    
@app.put("/", status_code=201)
async def put_kv(request: Request):
    
    payload = await request.json()
    prkv = PutReqKV(**payload).model_dump()
    w.write_record(kvs=prkv)
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=KV_SERVICE_URL,
                method="PUT",
                json=payload
            )
        return JSONResponse(content={"message":"Created"}, status_code=201)
    except:
        data = {"message":"Internal server error.", "status":"Failed"}
        return JSONResponse(content=data, status_code=500)
    
    