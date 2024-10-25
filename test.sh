#!/usr/bin/env bash

LOCAL_MIGRATIONS_DIR="./crates/db/migrations"
REMOTE_MIGRATIONS_DIR="migrations"

REMOTE_USER="root"
REMOTE_HOST="192.168.50.134"
REMOTE_PORT="22"

POSTGRES_CONTAINER_NAME="postgres"
DB_NAME="test_postgres"
DB_USER="postgres"
DB_PASSWORD="testpassword"
DB_HOST="192.168.50.134"
DB_PORT="5432"

CREATE_DB_CMD="docker exec -e PGPASSWORD=$DB_PASSWORD $POSTGRES_CONTAINER_NAME psql -U $DB_USER -h $DB_HOST -p $DB_PORT -c 'CREATE DATABASE $DB_NAME;'"

echo "Connecting to remote server and creating Postgresql database..."
ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "$CREATE_DB_CMD"

if [ $? -ne 0 ]; then
    echo "Failed to create database on remote server..."
    exit 1
else
    echo "Database created successfully on remote server..."
fi

echo "Running database migrations..."
if ssh "$REMOTE_USER@$REMOTE_HOST" "[ -d \"$REMOTE_MIGRATIONS_DIR\" ]"; then
    echo "$REMOTE_MIGRATIONS_DIR already exist on remote server"
else
    echo "$REMOTE_MIGRATIONS_DIR not found on remote server, creating now..."
    ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "mkdir -p $REMOTE_MIGRATIONS_DIR && chmod 755 $REMOTE_MIGRATIONS_DIR"
    if [ $? -eq 0 ]; then
        echo "$REMOTE_MIGRATIONS_DIR created successfully on remote server..."
    else
        echo "Failed to create $REMOTE_MIGRATIONS_DIR on remote server..."
        exit 1
    fi
fi

echo "Uploading migrations file to remote server..."
scp -r "$LOCAL_MIGRATIONS_DIR" "$REMOTE_USER@$REMOTE_HOST:"

if [ $? -eq 0 ]; then
    echo "Uploaded successfully to remote server..."
else
    echo "Failed to upload..."
    exit 1
fi

echo "Executing migrate..."


cd crates/ && dbmate -u "postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME?sslmode=disable" up

if [ $? -ne 0 ]; then
    echo "Failed to migrate database..."
    exit 1
else
    echo "migrate success..."
fi

echo "Running cargo test..."
cargo test


echo "Finishing with drop Postgresql database..."
DROP_DB_CMD="docker exec -e PGPASSWORD=$DB_PASSWORD $POSTGRES_CONTAINER_NAME psql -U $DB_USER -h $DB_HOST -p $DB_PORT -c 'DROP DATABASE $DB_NAME;'"

ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "$DROP_DB_CMD"

if [ $? -ne 0 ]; then
    echo "Failed to drop database on remote server..."
    exit 1
else
    echo "Database dropped successfully on remote server..."
fi

echo "Test completed."
