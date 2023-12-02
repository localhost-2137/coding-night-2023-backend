#!/bin/bash
# This script is used as entrypoint for the docker container.

if [ ! -f /app/db.db ]; then
    echo "Copying default database"
    cp /app/default.db /app/data/db.db
fi
/app/coding-night-2023-backend
