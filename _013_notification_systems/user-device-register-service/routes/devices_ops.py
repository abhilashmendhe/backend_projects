from fastapi import APIRouter, Depends, HTTPException, status
from database import get_connection
from fastapi.responses import JSONResponse
from models.device_model import RegisterDeviceRequest, GetDeviceResponse
from psycopg.rows import dict_row

router = APIRouter(prefix="/devices")

# curl -X POST localhost:56732/devices -H "Content-Type: application/json" -d '{"user_id":"Josah","token":"device_token_123","platform":"ios"}'
@router.post("", status_code=status.HTTP_201_CREATED)
async def register_device(request: RegisterDeviceRequest, conn = Depends(get_connection)):
    device = user = request.model_dump()
    try:
        async with conn.cursor() as cur:
            await cur.execute(
                """
                SELECT id FROM users WHERE username=%s
                """,
                (device["user_id"],),
            )
            userrow = await cur.fetchone()
            if not userrow:
                raise HTTPException(
                    status_code=status.HTTP_404_NOT_FOUND,
                    detail=f"User with id {device['user_id']} not found."
                )
                
            await cur.execute(
                """
                INSERT INTO devices(user_id,device_token,platform) 
                VALUES(%s,%s,%s)
                RETURNING id;
                """,
                (userrow[0],device["token"],device["platform"])
            )
            devicerow = await cur.fetchone()
        await conn.commit() 
        return {"message":f"device registered","id":devicerow[0],"device_token":device["token"]}
    except Exception as e:
        await conn.rollback()
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )
        
# curl -X GET localhost:56732/devices/device_token_123
@router.get("/{token}", status_code=status.HTTP_200_OK, response_model=GetDeviceResponse)
async def get_device(token: str, conn = Depends(get_connection)):
    
    try:
        async with conn.cursor(row_factory=dict_row) as cur:
            await cur.execute(
                """
                SELECT * FROM devices WHERE device_token = %s
                """,
                (token,),
            )
            row = await cur.fetchone()
            if not row:
                return JSONResponse({"message":f"device token {token} not found"}, status_code=status.HTTP_404_NOT_FOUND)
            return GetDeviceResponse.model_validate(row)
    except Exception as e:
            await conn.rollback()
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=str(e)
            ) 

# curl -X DELETE localhost:56732/devices/device_token_123            
@router.delete("/{token}", status_code=status.HTTP_200_OK)
async def delete_device_token(token: str, conn = Depends(get_connection)):
    
    try:
        async with conn.cursor(row_factory=dict_row) as cur:
            await cur.execute(
                """
                DELETE FROM devices WHERE device_token = %s
                RETURNING id,user_id,device_token,platform
                """,
                (token,),
            )
            row = await cur.fetchone()
            if not row:
                return JSONResponse({"message":f"device token {token} not found"}, status_code=status.HTTP_404_NOT_FOUND)
        return JSONResponse({"message":f"Device token: {token} deleted"})
    except Exception as e:
        await conn.rollback()
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )