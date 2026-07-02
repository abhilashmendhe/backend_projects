from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    USER_DEVICE_URL: str
    NOTIFICATION_SERVER_URL: str
    
    model_config = SettingsConfigDict(env_file=".env")
    
settings = Settings()