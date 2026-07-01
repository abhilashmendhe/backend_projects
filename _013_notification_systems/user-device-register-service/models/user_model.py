from pydantic import BaseModel, Field, EmailStr
from typing import Annotated, Optional
from datetime import datetime

class CreateUserRequest(BaseModel):
    username: Annotated[str, Field(..., description="username of a user")]
    email: Annotated[EmailStr, Field(..., description="email of a user")]
    
class GetUserResponse(BaseModel):
    id: Annotated[int, Field(description="ID of the user")]
    username: Annotated[str, Field(description="Username of the user")]
    email: Annotated[str, Field(description="Email of the user")]
    created_at: Annotated[
        datetime,
        Field(description="Timestamp when the user was created")
    ]