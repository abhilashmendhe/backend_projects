from models.users import *
from fastapi import APIRouter, HTTPException, Request
from fastapi.responses import JSONResponse
import httpx 
from lb import lb

router = APIRouter(prefix="/v1/user", tags=["users"])

@router.post("/", response_model=CreateUserResponse)
async def create_user(request: Request):
    # print(request)
    payload = await request.json()
    # create_user = CreateUserRequest(**payload).model_dump()
    # print(create_user)
    # print(await lb())
    port = await lb()
    myurl = f"http://localhost:{port}/v1/user"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=myurl,
                method="POST",
                json=payload
            )
        response.raise_for_status()
        resp_payload = response.json()
        create_user_resp = CreateUserResponse(**resp_payload)
        return create_user_resp
    except:
        data = {"message":"Internal server error.", "status":"Failed"}
        return JSONResponse(content=data, status_code=500)
    
@router.post("/login", response_model=LoginUserResponse)
async def login_user(request: Request):
    payload = await request.json()
    # print(payload)
    port = await lb()
    myurl = f"http://localhost:{port}/v1/user/login"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=myurl,
                method="POST",
                json=payload
            )

        response.raise_for_status()
        resp_payload = response.json()
        login_user_resp = LoginUserResponse(**resp_payload)
        return login_user_resp
    except httpx.RequestError as e:
        raise HTTPException(
            status_code=503,
            detail=f"User service unavailable: {str(e)}"
        )


@router.get("/{id}", response_model=UserModel)
async def get_user(id: int, request: Request):

    auth_header = request.headers.get("Authorization")
    print(auth_header)
    port = await lb()
    myurl = f"http://localhost:{port}/v1/user/{id}"
    print(myurl)
    # print(await request.json())
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=myurl,
                method="GET",
                headers={
                    "Authorization": auth_header
                }
            )
            # print(response)
        # response.raise_for_status()
        print(response.status_code)
        if response.status_code == 400:
            return JSONResponse(
                content={"message": "Bad request"},
                status_code=400
            )
        resp_payload = response.json()
        print(resp_payload)
        login_user_resp = UserModel(**resp_payload)
        return login_user_resp
    except httpx.RequestError as e:
        raise HTTPException(
            status_code=503,
            detail=f"User service unavailable: {str(e)}"
        )

@router.delete("/{id}")
async def delete_user(id: int, request: Request):
    auth_header = request.headers.get("Authorization")
    # print(auth_header)
    port = await lb()
    myurl = f"http://localhost:{port}/v1/user/{id}"

    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=myurl,
                method="DELETE",
                headers={
                    "Authorization": auth_header
                }
            )

        response.raise_for_status()
        return JSONResponse(content={'message':f'{response.text}'},status_code=204)
    except httpx.RequestError as e:
        raise HTTPException(
            status_code=503,
            detail=f"User service unavailable: {str(e)}"
        )