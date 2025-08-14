CREATE EXTENSION IF NOT EXISTS dblink;

DO
$$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_database WHERE datname = 'blogs'
   ) THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE blogs');
   END IF;
END
$$;

\connect blogs;

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL unique,
    password VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    token TEXT, 
    email VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT DEFAULT NULL,
    author_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    published BOOLEAN DEFAULT FALSE,
    CONSTRAINT fk_posts_author FOREIGN KEY (author_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT fk_comments_author FOREIGN KEY (author_id) REFERENCES users (id),
    CONSTRAINT fk_comments_post FOREIGN KEY (post_id) REFERENCES posts (id)
);