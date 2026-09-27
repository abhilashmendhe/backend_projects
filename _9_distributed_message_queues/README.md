# ⚠️ Discontinued - ✉️ ➡️ Distributed Message Queue

This project has been discontinued and is no longer maintained or actively developed.

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
