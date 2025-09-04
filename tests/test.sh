#!/usr/bin/env bash
set -euo pipefail

CONTAINER_NAME="pg_test_db"
POSTGRES_USER="postgres"
POSTGRES_PASSWORD="password"
POSTGRES_DB="postgres"
HOST_PORT=5432

# Stop & remove container if it already exists
if [ "$(podman ps -aq -f name=${CONTAINER_NAME})" ]; then
  echo "Stopping old postgres container..."
  podman rm -f ${CONTAINER_NAME} >/dev/null 2>&1 || true
fi

# Start a fresh Postgres container
echo "Starting postgres podman container..."
podman run -d \
  --name ${CONTAINER_NAME} \
  -e POSTGRES_USER=${POSTGRES_USER} \
  -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
  -e POSTGRES_DB=${POSTGRES_DB} \
  -p ${HOST_PORT}:5432 \
  postgres:latest

# Wait until Postgres is ready
echo "Waiting for postgres to be ready..."
until podman exec ${CONTAINER_NAME} pg_isready -U ${POSTGRES_USER} >/dev/null 2>&1; do
  sleep 1
done
echo "Postgres is ready."

# Load schema
echo "Loading database schema..."
podman cp ./tests/structure.sql ${CONTAINER_NAME}:/structure.sql
podman exec -u postgres ${CONTAINER_NAME} psql -d ${POSTGRES_DB} -f /structure.sql

# Load seed data
echo "Loading seed data..."
podman cp ./tests/seed.sql ${CONTAINER_NAME}:/seed.sql
podman exec -u postgres ${CONTAINER_NAME} psql -d ${POSTGRES_DB} -f /seed.sql

# Run cargo tests
echo "Running cargo tests..."
cargo test -- --color always --nocapture

# Cleanup
echo "Cleaning up..."
podman rm -f ${CONTAINER_NAME}
