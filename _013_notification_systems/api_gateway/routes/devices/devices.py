from fastapi import APIRouter, HTTPException, Request, status
from fastapi.responses import JSONResponse
import httpx 
from config import settings

router = APIRouter(prefix="/devices")

# curl -X POST localhost:8080/devices -H "Content-Type: application/json" -d '{"user_id":"Josah","token":"device_token_123","platform":"ios"}'
@router.post("")
async def register_device(request: Request):
    
    payload = await request.json()
    url = f"{settings.USER_DEVICE_URL}/devices"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=url,
                method="POST",
                json=payload
            )
            response.raise_for_status()
            resp_payload = response.json()
            return JSONResponse(resp_payload, status_code=status.HTTP_201_CREATED)
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )

# curl -X GET localhost:56732/devices/device_token_123
@router.get("/{token}")
async def get_device(token: str):
    
    url = f"{settings.USER_DEVICE_URL}/devices/{token}"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=url, 
                method="GET"
            )
            response.raise_for_status()
            resp_payload = response.json()
            return JSONResponse(resp_payload, status_code=status.HTTP_200_OK)
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )

# curl -X DELETE localhost:56732/devices/device_token_123
@router.delete("/{token}")
async def delete_device(token: str):
    
    url = f"{settings.USER_DEVICE_URL}/devices/{token}"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=url, 
                method="DELETE"
            )
            response.raise_for_status()
            resp_payload = response.json()
            return JSONResponse(resp_payload, status_code=status.HTTP_200_OK)
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )