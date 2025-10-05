from pydantic import BaseModel, Field
from typing import Annotated

class GetReqKV(BaseModel): 
    key: Annotated[str, Field(..., description="key of the KV store")]
    timestamp: Annotated[int, Field(..., description="timestamp of the value")]
    
class PutReqKV(BaseModel): 
    key: Annotated[str, Field(..., description="key of the KV store")]
    value: Annotated[str, Field(..., description="value of the key of the KV store")]
    timestamp: Annotated[int, Field(..., description="timestamp of the value")]