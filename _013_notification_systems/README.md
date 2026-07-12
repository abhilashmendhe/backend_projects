# Notification System

Building a notification system.

---

## High-level Design

<p style="text-align: center;">
<img src="images/Notification Systems High Level Design.png" 
    alt="HLD Notification System"
    style="float: left; margin-top: 20px;"
/>
    <em>Fig. 1 HLD of Notification System</em>
</p>

---

## Run

- To run gateway
$ uvicorn main:app --host 0.0.0.0 --port 8080 --reload

- To create user server
$ python3 -m uvicorn main:app --host 0.0.0.0 --port 56732 --reload

- To run notification server
$ cargo run -- --addr localhost -p 60001

- To run notification worker
$  cargo run -- --fetch-limit-jobs 1000 --platform ios --consumer-name consumer-1 --priority low --max-retry-count 3
