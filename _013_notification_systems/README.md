# 🔔 Notification System

Built a centralized notification service that enables applications to send reliable notifications through multiple channels using single API.

---

## Features

For now our system supports:

- 🚀 Unified API for sending notifications
- 📧 Multi-channel support (Email, SMS, Push, WhatsApp)
<!-- - 📝 Reusable notification templates -->
<!-- - ⏰ Scheduled and delayed notifications -->
- 🔄 Automatic retries with configurable backoff
- 📦 Bulk notification support
- 📊 Delivery status tracking
- 📈 Monitoring and metrics
<!-- - 🔐 Authentication and rate limiting -->
<!-- - ⚙️ Pluggable notification providers -->
<!-- - 📜 Audit logs and notification history -->
<!-- - 🌍 Multi-language template support -->
- 📨 Queue-based asynchronous processing
- 🔔 Event-driven architecture
<!-- - 🛡️ High availability with provider failover -->

---

## ⚙️ Architecture

<p style="text-align: center;">
<img src="images/Notification Systems High Level Design.png" 
    alt="HLD Notification System"
    style="float: left; margin-top: 20px;"
/>
    <em>Fig. 1 HLD of Notification System</em>
</p>

#### API Gateway
- Register users
- Register devices for existent users
- Forward notifications to notification server

#### Notification Server
- Create notifications
- Periodically pushes notification to queues (Redis streams)

#### Redis Streams
- Stores streams of notification data
- Two streams, one for high priority and second for low prirority
- AOF persistence 

#### Notification Worker
- Polls data from streams
- Periodically cleaning ACK data from streams
- Sends notificaiton to gateways (e.g. FCN, email, SMS, APN)

---

## ⛓️ Flow Diagram

#### Request Flow: Create Notification

```
Client
   │
POST /notify {`JSON Payload`}
   │
API Gateway
   │
Notification Server
   │
   |___ Async Ops
   |         |
   |    Store notification in PostgreSQL
   |         |
   |     Return Status
   |
   |___ Periodic Ops
             |
             |
        Push data to queues (streams)    
```

#### Worker Flow: Poll & Push

```
Notification Worker
    |
    |___  Poll
    |       |
    |     Push to Notification gateway
    |       |
    |     Response
    |       |
    |       |___ Status 202
    |       |
    |       |___ Status 410
    |       |        |_______ Update devices status = F   
    |       |
    |       |___ Status 429
    |       |        |_______ Retry-After (Header-Set Time)
    |       |   
    |       |___ Status 500 / 503
    |       |        |_______ Exponential retry backoff
    |       |
    |       |___ No response within 30s (drop)
    |
    |
    |___  Periodic Ops
            |
          Clean ACK data from streams
```

---

## Tech Stack

| Category | Technology |
|----------|------------|
| **Languages** | Python, Rust |
| **Web Frameworks** | Actix Web, FastAPI |
| **Database** | PostgreSQL |
| **Database Driver/ORM** | SQLx |
| **Queues/Streams** | Redis |
| **API Gateway** | Python + FastAPI |
| **Notification Server** | Rust + Actix Web |
| **Notification Worker** | Rust + Tokio + Reqwest |

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

1. **Clone the repo**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects

# 2. Go to _013_notification_systems
$ cd _013_notification_systems
```


## Run

- To run gateway
$ uvicorn main:app --host 0.0.0.0 --port 8080 --reload

- To create user server
$ python3 -m uvicorn main:app --host 0.0.0.0 --port 56732 --reload

- To run notification server
$ cargo run -- --addr localhost -p 60001 --background-fetch-limit-rows 500

- To run notification worker1 for ios low priority
$  cargo run -- --fetch-limit-jobs 200 --platform ios --consumer-name consumer-1 --priority low --max-retry-count 3
$  cargo run -- --fetch-limit-jobs 200 --platform ios --consumer-name consumer-2 --priority low --max-retry-count 3

- To run notification worker1 for ios high priority
$  cargo run -- --fetch-limit-jobs 100 --platform ios --consumer-name consumer-3 --priority high --max-retry-count 5
$  cargo run -- --fetch-limit-jobs 100 --platform ios --consumer-name consumer-4 --priority high --max-retry-count 5