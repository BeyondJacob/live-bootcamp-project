#!/bin/bash

# Check if PostgreSQL is running
if ! docker ps | grep -q "ps-db"; then
    echo "Starting PostgreSQL..."
    docker run --name ps-db -e POSTGRES_PASSWORD=mypostgrespassword123 -p 5432:5432 -d postgres:15.2-alpine
    sleep 3  # Wait for PostgreSQL to start
fi

# Check if Redis is running
if ! docker ps | grep -q "redis-db"; then
    echo "Starting Redis..."
    docker run --name redis-db -p "6379:6379" -d redis:7.0-alpine
    sleep 2  # Wait for Redis to start
fi

echo "Running auth-service..."
cargo run