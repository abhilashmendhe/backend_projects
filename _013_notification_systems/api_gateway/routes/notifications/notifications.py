from fastapi import APIRouter, HTTPException, Request, status
from fastapi.responses import JSONResponse
import httpx 
from config import settings
from models.notification_model import NotificationRequest
from fastapi.encoders import jsonable_encoder

router = APIRouter(prefix="/notify")

# $ curl -X POST localhost:8080/notify \
# > -H "Content-Type: application/json" \
# > -d '{"event_id": "evt_abc123", "recipient_user_id": "user_0042", "title": "Fall detected, Room 14", "body": "Mrs. Hansen, please check immediately", "priority": "high", "occurred_at": "2026-05-23T18:53:49Z"}'

@router.post("")
async def notify(request: NotificationRequest):
    payload = request.model_dump(mode="json")
    url = f"{settings.NOTIFICATION_SERVER_URL}/notify"
    try:
        async with httpx.AsyncClient() as client:
            response = await client.post(
                url=url,
                json=payload,
                headers = {"Content-Type": "application/json"}
            )
            response.raise_for_status()
            resp_payload = response.json()
            return JSONResponse(resp_payload, status_code=status.HTTP_201_CREATED)
    except httpx.HTTPStatusError as e:
        raise HTTPException(
            status_code=e.response.status_code,
            detail=e.response.text,
        )