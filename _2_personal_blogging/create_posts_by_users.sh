#!/bin/bash

URL="http://localhost:8080/v1/posts"

# Alice's posts
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTUzMjQ3NjcsInVzZXJuYW1lIjoiYWxpY2UifQ.a3NIz0Nry5g3rX2U2TS9KTyM7MuNZVAm-LdTfzjsRS8"

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Intro to Python", "content": "Learn why Python is widely used for automation, data science, and web development.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Mastering JavaScript", "content": "An in-depth look at asynchronous programming and building modern web apps.", "published": true, "login_required": true}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Data Analysis with Pandas", "content": "Explore data frames, cleaning, and transformation techniques with Pandas.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Building REST APIs with Flask", "content": "Step-by-step guide to creating and deploying REST APIs using Flask.", "published": true, "login_required": true}'

# Bob's posts
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTUzMjQ4MTEsInVzZXJuYW1lIjoiYm9iIn0.yBBpWaB5wA2F8_ccqvv6mtlhNWyDDzoZ8SvXlxDkBDk"

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Intro to Machine Learning", "content": "Understand supervised vs unsupervised learning and basic ML algorithms.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Deep Dive into SQL", "content": "Master complex queries, joins, and database optimization strategies.", "published": true, "login_required": true}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Getting Started with React", "content": "Learn the fundamentals of React components, hooks, and state management.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Docker for Beginners", "content": "Understand containerization and how to run applications with Docker.", "published": true, "login_required": true}'

# Carol's posts
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTUzMjQ4NDAsInVzZXJuYW1lIjoiY2Fyb2wifQ.rcJSGE5DrjVYoZEJIJ4JwkDcwp8c_gil6zTXdYIwpIo"

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Kubernetes Essentials", "content": "Learn how to orchestrate containers and manage deployments at scale.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Version Control with Git", "content": "A complete guide to branching, merging, and collaborating with Git.", "published": true, "login_required": true}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Building APIs with Node.js", "content": "Practical examples of designing scalable APIs using Express.js.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Django for Web Development", "content": "Learn how to build full-stack web applications with Django.", "published": true, "login_required": true}'

# Den's posts
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTUzMjQ5MTYsInVzZXJuYW1lIjoiZGVuIn0.D-4uFlkKbrGaDiWdgCCo9I2YX5tq9k_729NtYkm0whE"

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Cloud Basics with AWS", "content": "Introduction to core AWS services like EC2, S3, and IAM.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Cybersecurity Fundamentals", "content": "Understand threats, vulnerabilities, and defense mechanisms.", "published": true, "login_required": true}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Intro to Data Visualization", "content": "Learn how to create impactful charts and dashboards with Python libraries.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "C++ Programming Basics", "content": "C++ syntax, OOP concepts, and building efficient applications.", "published": true, "login_required": true}'

# Eva's posts
TOKEN="eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3NTUzMjQ5NDAsInVzZXJuYW1lIjoiZXZhIn0.8J1SkxTlWpViXpy_z8SNK_EL8zXghFSUaqibdkXaxKM"

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Functional Programming with Scala", "content": "An introduction to functional programming principles in Scala.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Artificial Intelligence Overview", "content": "Explore the concepts of AI, NLP, computer vision, and expert systems.", "published": true, "login_required": true}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "Agile Software Development", "content": "Scrum, Kanban, and agile best practices for modern software teams.", "published": true, "login_required": false}'

curl -X POST $URL -H "Content-Type: application/json" -H "x-auth-token: $TOKEN" -d '{"title": "DevOps Fundamentals", "content": "CI/CD pipelines, infrastructure as code, and DevOps culture explained.", "published": true, "login_required": true}'
