#!/bin/bash

# Stop and remove existing container if it exists
docker stop postgres-dev 2>/dev/null
docker rm postgres-dev 2>/dev/null

# Run new PostgreSQL container
docker run --name postgres-dev \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  -d postgres

echo "PostgreSQL container 'postgres-dev' is now running."
echo "Connection string: postgres://postgres:password@localhost:5432/postgres"