from fastapi import APIRouter, Depends, status, HTTPException
from fastapi.responses import JSONResponse
from database import get_connection
from models.user_model import CreateUserRequest, GetUserResponse
from psycopg.rows import dict_row

router = APIRouter(prefix="/user")

# curl -X POST localhost:56732/user -H "Content-Type: application/json" -d '{"username":"Josah","email":"josh@gma.com"}'
@router.post("", status_code=status.HTTP_201_CREATED)
async def create_user(request: CreateUserRequest, conn = Depends(get_connection)):
    user = request.model_dump()
    
    try:
        async with conn.cursor() as cur:
            await cur.execute(
                """
                INSERT INTO users (username, email) 
                VALUES(%s, %s) 
                RETURNING id;
                """,
                (user["username"],user["email"]),
            )
            row = await cur.fetchone()
        await conn.commit()
        return {"message":"User created", "id": row[0]}
    except Exception as e:
        await conn.rollback()
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )

# curl -X GET localhost:56732/user/1'
@router.get("/{id}", status_code=status.HTTP_200_OK, response_model=GetUserResponse)
async def get_user(id: int, conn = Depends(get_connection)):
    
    try:
        async with conn.cursor(row_factory=dict_row) as cur:
            await cur.execute(
                """SELECT id,username,email,created_at FROM users WHERE id = %s""",
                (id,),
            )
            row = await cur.fetchone()
            if not row:
                return JSONResponse({"message":f"User with id:{id} not found!"}, status_code=status.HTTP_404_NOT_FOUND)
        return GetUserResponse.model_validate(row)
    except Exception as e:
        await conn.rollback()
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e)
        )    