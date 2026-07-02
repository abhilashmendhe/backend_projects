from fastapi import APIRouter, HTTPException, Request, status
from fastapi.responses import JSONResponse
import httpx 
from config import settings

router = APIRouter(prefix="/user")

# curl -X POST localhost:8080/user -H "Content-Type: application/json" -d '{"username":"Josah","email":"josh@gma.com"}'
@router.post("")
async def create_user(request: Request):
    
    payload = await request.json()
    url = f"{settings.USER_DEVICE_URL}/user"
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

@router.get("/{id}", status_code=status.HTTP_200_OK)
async def get_user(id: int):
    
    url = f"{settings.USER_DEVICE_URL}/user/{id}"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=url,
                method="GET",
            )
            response.raise_for_status()
            resp_payload = response.json()
            return JSONResponse(resp_payload, status_code=status.HTTP_200_OK)
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )
