# ✉️ ➡️ Distributed Message Queue

Starting to build a distributed message queue service.....

---
## Architecture flow
```text
   +-----------+
   | Producer  |
   +-----------+
         |
      HTTP/TCP
         |
         v
 +------------------+
 |   QueueServer    |
 +------------------+
 | API Layer        |
 | Queue Manager    |
 | Redis Client     |
 +------------------+
         |
         v
 +------------------+
 |      Redis       |
 +------------------+
         |
         v
 +------------------+
 |     Worker       |
 +------------------+
 ```