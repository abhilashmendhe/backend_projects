# Blogging Post (Rust + Axum + SQLx)

A blogging backend web application built in Rust using the [Axum](https://github.com/tokio-rs/axum) web framework for routing and [SQLx](https://github.com/launchbadge/sqlx) for database connectivity and schema management.   

It provides APIs for:
* User authentication & management
* Blog post creation and updates
* Nested comment management on posts

## Features

* **Users**
  * Register, login, update password
  * Logout (requires authentication)
  * Delete account (requires authentication)

* **Posts**
  * Create, update, delete posts (auth required)
  * Fetch all posts or a specific post

* **Comments**
  * Add comments to posts (auth required)
  * Update or delete specific comments (auth required)
  * Fetch comments for a post

* **Other**
  * Health check endpoint (/health)
---
## API Endpoints

### Public Routes
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET`  | `/health` | Health check |
| `POST` | `/v1/users` | Register a new user |
| `POST` | `/v1/users/login` | Login user |
| `PUT`  | `/v1/users` | Update user password |
| `GET`  | `/v1/posts` | Fetch all posts |
| `GET`  | `/v1/posts/{post_id}` | Fetch a specific post |

### Protected Routes (require authentication)
| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/users/logout` | Logout current user |
| `DELETE` | `/v1/users` | Delete user account |
| `POST` | `/v1/posts` | Create a new post |
| `PATCH` | `/v1/posts/{post_id}` | Update a post |
| `DELETE` | `/v1/posts/{post_id}` | Delete a post |
| `GET` | `/v1/posts/{post_id}/comments` | Fetch comments for a post |
| `POST` | `/v1/posts/{post_id}/comments` | Add a comment to a post |
| `PATCH` | `/v1/posts/{post_id}/comments/{comment_id}` | Update a comment |
| `DELETE` | `/v1/posts/{post_id}/comments/{comment_id}` | Delete a comment |

---

## Example Requests (with `curl`)

> Replace `TOKEN` with your authentication token.  

### Public
```bash
# Health check
curl http://localhost:3000/health

# Create a user
curl -X POST http://localhost:8080/v1/users 
  -H "Content-Type: application/json" 
  -d '{
    "username": "alice",
    "password": "alice123",
    "email": "alice123@gmail.com"
  }'

# Login
curl -X POST http://localhost:3000/v1/users/login      
    -H "Content-Type: application/json"      
    -d '{"username": "alice", "password": "secret"}'

# Update password
curl -X PUT http://localhost:3000/v1/users      
    -H "Content-Type: application/json"      
    -d '{"username": "alice", old_password": "secret", "new_password": "new_secret"}'

# Fetch all posts (Without token - Returns all non login posts)
curl http://localhost:3000/v1/posts

# Fetch all posts (With auth header token - Returns all posts)
curl http://localhost:3000/v1/posts
    -H "x-auth-token: $TOKEN"

# Fetch a specific post (Returns if login is not required without token)
curl http://localhost:3000/v1/posts/1

# Fetch a specific post (Returns post)
curl http://localhost:3000/v1/posts/1
    -H "x-auth-token: $TOKEN"

```

### Protected
```bash
# Logout
curl -X POST http://localhost:3000/v1/users/logout      
    -H "x-auth-token: $TOKEN"

# Delete user
curl -X DELETE http://localhost:3000/v1/users     
    -H "x-auth-token: $TOKEN"

# Create a post
curl -X POST http://localhost:3000/v1/posts      
    -H "Content-Type: application/json" 
    -H "x-auth-token: $TOKEN" 
    -d '{
    "title": "Intro to Python", 
    "content": "Learn why Python is widely used for automation, data science, and web development.", 
    "published": true, 
    "login_required": false
    }'


# Update a post
curl -X PATCH http://localhost:3000/v1/posts/1
    -H "Content-Type: application/json" 
    -H "x-auth-token: $TOKEN" 
    -d '{
    "title": "Intro to Python", 
    "content": "Learn why Python is widely used for automation, data science, and web development.", 
    "published": true, 
    "login_required": false
    }'

# Delete a post
curl -X DELETE http://localhost:3000/v1/posts/1
    -H "x-auth-token: $TOKEN
    
# Fetch comments for a post
curl -X GET http://localhost:3000/v1/posts/1/comments
    -H "x-auth-token: $TOKEN
    
# Add a comment
curl -X POST http://localhost:3000/v1/posts/1/comments
    -H "Content-Type: application/json"
    -H "x-auth-token: $TOKEN
    -d '{"content": "Nice post!"}'

# Update a comment
curl -X PATCH http://localhost:3000/v1/posts/1/comments/2
    -H "Content-Type: application/json"
    -H "x-auth-token: $TOKEN
    -d '{"content": "Nice post!"}'

# Delete a comment
curl -X DELETE http://localhost:3000/v1/posts/1/comments/2
    -H "x-auth-token: $TOKEN
```

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Docker](https://www.docker.com/)

### Run Locally
```bash
# Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_2_personal_blogging
$ cd _1_todo_app

# Run the server
$ cargo run

# Run docker
$ docker compose up -d database
$ docker compose run --rm db-init
```

The server will be available at `http://localhost:8080`.

---

## License
MIT License © 2025 Abhilash Mendhe

