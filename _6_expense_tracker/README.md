# 🛒💸💰 Expense Tracker

Built a basic *expense tracker* application that can track of different categories like food, shopping, health, housing etc. 

## 🚀 Features

* **Users**
    * Register, and login
    * Logout (requires JWT/token-based authentication)
    * Delete account (requires JWT/token-based authentication)
    * Account activation
* **Expenses (requires JWT/token-based authentication)**
    * Create, update, and delete expenses 
    * Get a single expense by id 
    * Get all expenses:
        * [x] Filter by date range
        * [x] Filter by category
        * [x] Filter by amount range
        * [ ] Search by text (note/description)
        * [ ] Pagination + sorting

--- 

## 🛠️ Tech Stack
* Rust
* Actix Web
* SQLx
* PostgreSQL

--- 

## API Endpoints

### Users Routes
| Method | Endpoint | Description | Protected |
|--------|----------|-------------|-----------|
| `POST` | `/v1/users` | Register a new user | ❌ |
| `POST` | `/v1/users/login` | Login user | ❌ |
| `PUT`  | `/v1/users/activate` | Account activation | ❌ |
| `GET`  | `/v1/users/{id}` | Get user by id | ✅ |
| `DELETE` | `/v1/users/{id}` | Delete user account | ✅ |
| `POST` | `/v1/users/logout` | Logout current user | ✅ |
| `PUT` | `/v1/users/update-password` | Update password | ✅ |

### Expense Routes
| Method | Endpoint | Description | Protected |
|--------|----------|-------------|-----------|
| `POST` | `/v1/expenses` | Create an expense | ✅ |
| `GET` | `/v1/expenses` | Get all expense | ✅ |
| `GET` | `/v1/expenses/{id}` | Get expense by id | ✅ |
| `PUT`  | `/v1/expenses/{id}` | Update expense by id | ✅ |
| `DELETE` | `/v1/expenses/{id}` | Delete expense by id | ✅ |

#### 🔑 Authentication

Protected routes require authentication.

Example header:

```bash
Authorization: Bearer <token>
```

---

## Getting started

#### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [docker](https://www.docker.com/)

#### Setup Docker
```bash
# 1. Pull docker images
$ docker pull postgres:16

# 2. Run docker
$ docker compose up -d

# 3. Stop docker
$ docker compose down
```
#### Clone Repo
```bash
# 1. Clone Repo
$ git clone https://github.com/abhilashmendhe/backend_projects

# 2. Go to _3_weather_api_service 
$ cd _6_expense_tracker
```

#### Setup .env file
```bash
# 1. Open .env
$ vim .env

# 2. Edit .evn
POSTGRES_USER=<postgres>
POSTGRES_PASSWORD=<password>
POSTGRES_DB=expensesdb
DATABASE_URL=postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:5433/<POSTGRES_DB>
SECRET=<MY SECRET>
~                                                
~                                                
~    
# 3. Save and exit
```

#### Backend Setup
```bash
# 1. Start backend server
$ cargo run 

OR

# 2. To watch with run
$ cargo watch -q -c -w src/ -x run
```

---

## Example Requests (with `curl`)

> Replace `TOKEN` with your authentication token.  

#### User Requests
```bash
# Health check
curl http://localhost:8080/health

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
    -d '{"username": "alice", "password": "alice123"}'

# Update password
curl -X PUT http://localhost:3000//v1/users/update-password      
    -H "Content-Type: application/json"  
    -H "x-auth-token: $TOKEN"    
    -d '{"password": "new-password"}'

# Get a user by id
curl -X GET http://localhost:3000/v1/users/{id}
    -H "x-auth-token: $TOKEN"    

# Delete a user by id
curl -X DELETE http://localhost:3000/v1/users/{id}
    -H "x-auth-token: $TOKEN"    

# Get a user by id
curl -X POST http://localhost:3000/v1/users/logout
    -H "x-auth-token: $TOKEN"    

# # Activate user
# curl -X PUT http://localhost:3000/v1/users/update-password
#     -H "Content-Type: application/json"  
#     -d '{
#         "id": "2",
#         "username": "alice123",
#         "password": "alice123",
#     }'
```

#### Expense Requests
```bash
# 1. Create an expense
curl -X POST http://localhost:8080/v1/expenses 
  -H "Content-Type: application/json" 
  -H "x-auth-token: $TOKEN"   
  -d '{
    "category_name": "travel",
    "amount": "10.50",
    "description": "uber from home to office"
    "expense_date": "10-04-2026"
  }'

# 2. Get all expenses (can be filtered by query param)
curl -X GET http://localhost:8080/v1/expenses
  -H "x-auth-token: $TOKEN"   

#   2.1. Filter by date range
curl -X GET http://localhost:8080/v1/expenses?start_date=01-01-2026&end_date=31-01-2026
  -H "x-auth-token: $TOKEN"   

#   2.2. Filter by category
curl -X GET http://localhost:8080/v1/expenses?category=food
  -H "x-auth-token: $TOKEN"

#   2.3. Filter by amount range
curl -X GET http://localhost:8080/v1/expenses?min_amount=10.50&max_amount=50.30
  -H "x-auth-token: $TOKEN"

# 3. Get expense by id
curl -X GET http://localhost:8080/v1/expenses/2
  -H "x-auth-token: $TOKEN"

# 4. Update expense by id
curl -X PUT http://localhost:8080/v1/expenses/2
  -H "x-auth-token: $TOKEN"

# 5. Delete an expense by id
curl -X DELETE http://localhost:8080/v1/expenses/2
  -H "x-auth-token: $TOKEN"
```

---

## License
MIT License © 2026 Abhilash Mendhe

