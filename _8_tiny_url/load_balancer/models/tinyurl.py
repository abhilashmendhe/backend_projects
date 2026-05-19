from pydantic import BaseModel, Field
from typing import Annotated, Optional
from datetime import datetime

class TinyUrlModel(BaseModel):
    id: Annotated[int, Field(..., description="id of tinyurl")]
    user_id: Annotated[int, Field(..., description="id of a user refer to tinyurl")]
    short_url_code: Annotated[str, Field(..., description="short url code")]
    long_url: Annotated[str, Field(..., description="long url")]
    created_at: Annotated[Optional[datetime], Field(..., description="Date time field of a tinyurl to create")]
    deleted_at: Annotated[Optional[datetime], Field(..., description="Date time field of a tinyurl to delete")]
    expired_at: Annotated[Optional[datetime], Field(..., description="Date time field of a tinyurl to update")]

class TinyUrlCreateReq(BaseModel):
    long_url: Annotated[str, Field(..., description="long url")]
    expired_at: Annotated[Optional[datetime], Field(..., description="Date time field of a tinyurl to update")]

class TinyUrlCreateResp(BaseModel):
    short_url: Annotated[str, Field(..., description="short url code")]