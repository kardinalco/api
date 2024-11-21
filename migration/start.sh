#!/bin/bash
sea-orm-cli migrate -u "$DATABASE_URL"
echo "done" > /shared/migrations_done