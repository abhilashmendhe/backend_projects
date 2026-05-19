from models.tinyurl import *
from fastapi import APIRouter, HTTPException, Request
from fastapi.responses import JSONResponse, RedirectResponse, HTMLResponse
import httpx 
from lb import lb

router = APIRouter(prefix="/v1/tinyurl", tags=["tinyurl"])

@router.post("/", response_model=TinyUrlCreateResp)
async def create_tinyurl(request: Request):
    auth_header = request.headers.get("Authorization")
    payload = await request.json()
    port = await lb()
    myurl = f"http://localhost:{port}/v1/tinyurl"
    # print("url",myurl)
    # print("create tinyurl: ",payload)
    try:
        async with httpx.AsyncClient() as client:
            response = await client.request(
                url=myurl,
                method="POST",
                json=payload,
                headers={
                    "Authorization": auth_header
                }
            ) 
        # print(response)
        response.raise_for_status()
        resp_payload = response.json()
        tinyurl_create_resp = TinyUrlCreateResp(**resp_payload)
        return tinyurl_create_resp
    except httpx.RequestError as e:
        raise HTTPException(
            status_code=503,
            detail=f"User service unavailable: {str(e)}"
        )

@router.get("/{key}")
async def get_redirect(key: str, request: Request):
    # print("in get_redirect")
    auth_header = request.headers.get("Authorization")
    port = await lb()
    myurl = f"http://localhost:{port}/v1/tinyurl/{key}"
    # print(auth_header)
    try:
        async with httpx.AsyncClient(follow_redirects=True) as client:
            response = await client.request(
                url=myurl,
                method="GET",
                headers={
                    "Authorization": auth_header
                }
            )
            
        ### -------------- redirect = True ---------------
        # if response.status_code in [301, 302, 307, 308]:

        #     redirect_url = response.headers.get("Location")

        #     return RedirectResponse(
        #         url=redirect_url,
        #         status_code=response.status_code
        #     )

        # response.raise_for_status()

        # return response.json()

        ### -------------- redirect = True ---------------
        response.raise_for_status()
        return HTMLResponse(
            content=response.text,
            status_code=response.status_code
        )
    except httpx.RequestError as e:
        raise HTTPException(
            status_code=503,
            detail=f"User service unavailable: {str(e)}"
        )