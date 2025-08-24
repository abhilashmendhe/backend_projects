# Task Manager/Todo List Backend (Rust + Axum + SeaORM)

This is a backend service application built in Rust using the [Axum](https://github.com/tokio-rs/axum) web framework for routing and [SeaORM](https://www.sea-ql.org/SeaORM) for database connectivity and schema management. This simple web services provides task management APIs to add/remove tasks like a ToDo list application with user authentication.  

## Features

* **User Management**
  * User registration
  * Login with authentication
  * Logout with session/token invalidation

* **Task Management**
  * Create, update, delete tasks
  * Mark tasks as completed or uncompleted
  * Soft delete tasks (preserve data instead of permanent removal)
  * Fetch all tasks or a single task

* **Authentication**
  * Protected task routes using middleware (`require_auth`)
  * Only authenticated users can perform task operations

---

## API Endpoints

### Public Routes
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET`  | `/` | Health check / Hello world |
| `POST` | `/api/v1/users` | Register a new user |
| `POST` | `/api/v1/users/login` | Login user |

### Protected Routes (require authentication)
| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/users/logout` | Logout user |
| `POST` | `/api/v1/tasks` | Create a new task |
| `GET`  | `/api/v1/tasks` | Get all tasks |
| `GET`  | `/api/v1/tasks/{id}` | Get a specific task |
| `PUT`  | `/api/v1/tasks/{id}/completed` | Mark task as completed |
| `PUT`  | `/api/v1/tasks/{id}/uncompleted` | Mark task as uncompleted |
| `PATCH`| `/api/v1/tasks/{id}` | Update a task |
| `DELETE` | `/api/v1/tasks/{id}` | Soft delete a task |

---

## Example Requests (with `curl`)

> Replace `TOKEN` with your actual authentication token.

### Public
```bash
# Health check
curl http://localhost:3000/

# Register a user
curl -X POST http://localhost:3000/api/v1/users      
    -H "Content-Type: application/json"      
    -d '{"username": "alice", "password": "secret"}'

# Login
curl -X POST http://localhost:3000/api/v1/users/login      
    -H "Content-Type: application/json"      
    -d '{"username": "alice", "password": "secret"}'
```

### Protected (requires `Authorization: Header TOKEN`)

```bash
# Logout
curl -X POST http://localhost:3000/api/v1/users/logout      
     -H "x-auth-token: $TOKEN"

# Create a task
curl -X POST http://localhost:3000/api/v1/tasks      
     -H "x-auth-token: $TOKEN"      
     -H "Content-Type: application/json"      
     -d '{
        "title": "new task 106",
        "description": "this aa task is of abhi",
        "priority": "A"
    }'

# Get all tasks 
curl -X GET http://localhost:3000/api/v1/tasks 
    -H "x-auth-token: $TOKEN"

# Get one task
curl -X GET http://localhost:3000/api/v1/tasks/1
    -H "x-auth-token: $TOKEN"

# Mark completed
curl -X PUT http://localhost:3000/api/v1/tasks/1/completed    
    -H "x-auth-token: $TOKEN"
    
# Mark uncompleted
curl -X PUT http://localhost:3000/api/v1/tasks/1/uncompleted 
    -H "x-auth-token: $TOKEN"
    
# Update a task
curl -X PATCH http://localhost:3000/api/v1/tasks/1
    -H "x-auth-token: $TOKEN"      
    -H "Content-Type: application/json"      
    -d '{
    "title": "new task 106",
    "description": "this aa task is of abhi",
    "priority": "A"
    }'

# Soft delete a task
curl -X DELETE http://localhost:3000/api/v1/tasks/1
     -H "x-auth-token: $TOKEN"      
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
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_1_todo_app
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

