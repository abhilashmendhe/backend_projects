# TinyURL

Built a URL shortneing web service that provides short aliases for redirection of long URLs. The idea was inspired by **bitly**.

---

## 🚀 Features

- URL shortening
- URL redirection
- Load balancer
- Bloom filter to reduce unnecessary database lookups
- Horizontal scaling
- REST APIs
- Create users

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/users` | Register a new user |
| `POST` | `/v1/users/login` | Login user |
| `POST` | `/v1/tinyurl` | Create a short URL |
| `GET`  | `/v1/tinyurl/{key}` | Get a short URL | 


---

## ⚙️ Architecture

<p style="text-align: center;">
<img src="images/tinyurl-arch-design.png" 
    alt="Tiny url design"
    style="float: left; margin-top: 20px;"
/>
    <em>Fig. 1 HLD of Tiny Url</em>
</p>

#### Load Balancer
- Distributes incoming requests to tinyurl servers
- A simple Round-robin algorithm to distribute load

#### TinyURL Server
- Creating short URLs
- Redirecting users
- Communicating with Bloom filter
- Reading/writing database

#### Bloom Filter Server 
- Fast existence checks
- Reducing database hits

---

## Flow Diagrams

#### Request Flow: Create Short URL

```
Client
   │
POST /v1/tinyurl
   │
Load Balancer
   │
TinyURL Server
   │
Generate shortcode
   │
Store in PostgreSQL
   │
Update Bloom Filter
   │
Return short URL
```

#### Request Flow: Redirect

```
Client
   │
GET /v1/tinyurl/abc123
   │
Load Balancer
   │
TinyURL Server
   │
Bloom Filter
   │
   ├── Not Present
   │      │
   │      └── 404
   │
   └── Present
          │
          ▼
      Cache Lookup
          │
          ├── Hit
          │    │
          │    └── Redirect
          │
          └── Miss
               │
               ▼
          PostgreSQL
               │
          Update Cache
               │
            Redirect
```

---

## Tech Stack

| Category | Technology |
|----------|------------|
| **Languages** | Python, Rust |
| **Web Frameworks** | Actix Web, Axum, FastAPI |
| **Database** | PostgreSQL |
| **Database Driver/ORM** | SQLx |
| **Cache** | Redis |
| **Load Balancer** | Python + FastAPI |
| **TinyURL Service** | Rust + Actix Web |
| **Bloom Filter Service** | Rust + Axum |

---

## Getting started

#### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Python](https://www.python.org/downloads/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [docker](https://www.docker.com/)


#### Setup Docker
```bash
# 1. Pull postgres image
$ docker pull postgres:16

# 2. Pull docker redis:alpine image
$ docker pull redis:7-alpine # lightweight image

# 3. Run docker
$ docker compose up -d

# 4. Stop docker
$ docker compose down
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

1. **Clone the repo**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects

# 2. Go to _8_tiny_url
$ cd _8_tiny_url
```

2. **Setup Python environment and install Python packages and execute load balancer**
```bash
# 1. Go to load_balancer folder (Python based)
$ cd load_balancer

# 2. Create virtual environment
$ python -m venv venv

# 3. Activate virtual environment
$ source venv/bin/activate

# 4. Install pip packages from requirements.txt file
$ pip install -r requirements.txt

# 5. Now run
$ uvicorn main:app --host 0.0.0.0 --port 8000 --reload
```

3. **Run TinyURL Server**
```bash
# 1. Go to tu_server folder (Rust based)
$ cd tu_server # our tinyurl server

# 3. Run
$ cargo run -- --addr localhost -p 8080 --redis-addr localhost --redis-port 6379
```

4. **Run Bloom Filter Server**
```bash
# 1. Go to bf_server folder (Rust based)
$ cd bf_server # our bloom filter server

# 2. Run
$ cargo run -- --addr localhost -p 8080 --bf-server-workers 4
```


---

## References

1. <a id="ref-1"></a> bytebytego.com - *[Design A URL Shortener](https://bytebytego.com/courses/system-design-interview/design-a-url-shortener)*

2. <a id="ref-2"></a> hellointerview.com - *[Bitly](https://www.hellointerview.com/learn/system-design/problem-breakdowns/bitly)*