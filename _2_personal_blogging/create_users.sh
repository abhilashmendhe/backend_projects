#!/bin/bash

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "alice123",
    "email": "alice123@gmail.com"
  }'

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "bob",
    "password": "bob123",
    "email": "bob123@gmail.com"
  }'

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "carol",
    "password": "carol123",
    "email": "carol123@gmail.com"
  }'

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "den",
    "password": "den123",
    "email": "den123@gmail.com"
  }'

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "eva",
    "password": "eva123",
    "email": "eva123@gmail.com"
  }'

curl -X POST http://localhost:8080/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "farah",
    "password": "farah123",
    "email": "farah123@gmail.com"
  }'