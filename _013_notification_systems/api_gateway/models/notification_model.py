from pydantic import BaseModel, Field
from typing import Annotated
from datetime import datetime

class NotificationRequest(BaseModel):
    event_id: Annotated[str, Field(description="id of an event which is globally unique")]
    recipient_user_id: Annotated[str, Field(description="username of a user")]
    title: Annotated[str, Field(description="title of an event")]
    body: Annotated[str, Field(description="message body of an event")]
    priority: Annotated[str, Field(description="priority of an event. `high` or `low`")]
    occurred_at: datetime = Field(description="Timestamp of an event occurred/created")
    