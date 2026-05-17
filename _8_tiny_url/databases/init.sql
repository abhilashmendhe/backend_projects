CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY, 
    username VARCHAR(64) NOT NULL UNIQUE, 
    password VARCHAR(255) NOT NULL, 
    email    VARCHAR(64) NOT NULL, 
    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ, 
    token TEXT
);

CREATE TABLE IF NOT EXISTS tinyurl (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL, 
    short_url_code VARCHAR(10) NOT NULL, 
    long_url TEXT NOT NULL, 
    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    expired_at TIMESTAMPTZ,

    CONSTRAINT fk_tinyurl_user
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,

    UNIQUE(short_url_code),
    UNIQUE(user_id, long_url)
);

CREATE INDEX idx_tinyurl_user_id ON tinyurl(user_id);
