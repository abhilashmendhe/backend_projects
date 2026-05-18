from pydantic import BaseModel, Field
from typing import Annotated, Optional
from datetime import datetime

class CreateUserRequest(BaseModel):
    username: Annotated[str, Field(..., description="username to create user in DB")]
    password: Annotated[str, Field(..., description="password to create user in DB")]
    email:    Annotated[str, Field(..., description="email to create user in DB")]

class CreateUserResponse(BaseModel):
    id: Annotated[int, Field(..., description="Id of a user")]
    created_at: Annotated[datetime, Field(..., description="Date time field of a user")]
    token: Annotated[Optional[str], Field(None, description="token of a user")]

class LoginUserRequest(BaseModel):
    username: Annotated[str, Field(..., description="username to login user in DB")]
    password: Annotated[str, Field(..., description="password to login user in DB")]

class LoginUserResponse(BaseModel):
    id: Annotated[int, Field(..., description="Id of a user")]
    username: Annotated[str, Field(..., description="username to login user in DB")]
    token: Annotated[str, Field(..., description="token of a user")]

class UserModel(BaseModel):
    id: Annotated[int, Field(..., description="Id of a user")]
    username: Annotated[str, Field(..., description="username to create user in DB")]
    password: Annotated[str, Field(..., description="password to create user in DB")]
    email:    Annotated[str, Field(..., description="email to create user in DB")]
    created_at: Annotated[datetime, Field(..., description="Date time field of a user to create")]
    deleted_at: Annotated[Optional[datetime], Field(..., description="Date time field of a user to delete")]
    token: Annotated[Optional[str], Field(None, description="token of a user")]