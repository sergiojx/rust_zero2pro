#!/usr/bin/env bash
# Why 10.0.2.2 works in vagrant is still not clear
DATABASE_URL=postgres://postgres:password@10.0.2.2:5432/newsletter
export DATABASE_URL
## sqlx database create
# This will execute the migration commands in project/migrations
# Expected output
# ./scripts/vagrant_sqlx_migrate_db.sh 
# Applied 20240713155313/migrate create subscriptions table (28.518189ms)
sqlx migrate run