# ⏲️ Timebased KV-Store

This is a basic time-based key-value store that stores multiple values for the same key but for different timestamps that is monotonically increasing. This project is build on `Rust` and `Python`.

---

##  Features

The KV-store can support/store multiple `values` for a specific `key` based on the `timestamp`.

* **PUT**
    * Insert a `value` with the latest `timestamp` for a specific `key`.
    * Can insert another value with latest `timestamp` for the same `key`.

* **GET**
    * Fetch a `value` by sending `key` and a `timestamp`.
    * If `value` exists for the current `timestamp`, it returns the `value`.
    * If there are multiple such `values`, returns the value with the largest `timestamp`.
    * If no value exists for the timestamp _or_ key exists, returns `""`.

---

## High-level Design

<p style="text-align: center;">
<img src="images/TimebasedKV.png" 
    alt="Timbe-based KV store design"
    style="float: left; margin-top: 20px;"
/>
    <em>Fig. 1 HLD of Timebased KV Store</em>
</p>

---

## Getting started

#### Prerequisites
- [Python](https://www.python.org/downloads/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

#### Install packages

1. **Setup Python environment and install Python packages**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_5_kv_stores
$ cd _5_kv_stores

# 2. Go to api_gateway folder (Python based)
$ cd api_gateway

# 3. Create virtual environment
$ python -m venv venv

# 4. Activate virtual environment
$ source venv/bin/activate

# 5. Install pip packages from requirements.txt file
$ pip install -r requirements.txt

```

2. **Setup Rust**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_5_kv_stores
$ cd _5_kv_stores

# 2. Go to kv_store folder (Rust based)
$ cd kv_store

# 3. Build
$ cargo build

```

#### Execution

1. **Run Key-Value Store (Rust) Server**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_5_kv_stores
$ cd _5_kv_stores

# 2. Go to kv_store folder (Rust based)
$ cd kv_store

# 3. Run
$ cargo run -- --ip-addr 0.0.0.0 --port 58322
🚀 Timestore KV Started listening on `0.0.0.0:58322`


```

1. **Run API gateway (Python)**
```bash
# 1. Clone the repo
$ git clone https://github.com/abhilashmendhe/backend_projects/tree/main/_5_kv_stores
$ cd _5_kv_stores

# 2. Go to api_gateway folder (Python based)
$ cd api_gateway

# 3. Run
$ KV_SERVICE_URL="http://0.0.0.0:58322/api/v1" \
  uvicorn main:app --host 0.0.0.0 --port 8000

INFO:     Started server process [185394]
INFO:     Waiting for application startup.
🚀 Starting up....
INFO:     Application startup complete.
INFO:     Uvicorn running on http://0.0.0.0:8000 (Press CTRL+C to quit)

```


---  

## Example Requests (with `curl`)

```bash

# PUT Request - Insert a KV with timestamp
$ curl -X PUT http://localhost:8080 
    -H 'Content-Type: application/json' 
    -d '{
       "key": "mykey", 
       "value": "myvalue", 
       "timestamp" : 1673524092123456
     }'

# GET Request - Get a value for a specific timestamp
$ curl -X GET http://localhost:8080
    -H 'Content-Type: application/json' 
    -d '{
       "key": "mykey", 
       "timestamp" : 1673524092123456
     }'
```

---  

## Future Work

- [ ] Make high availability.
    - Implement consistent hashing.

---

## License
MIT License © 2025 Abhilash Mendhe
