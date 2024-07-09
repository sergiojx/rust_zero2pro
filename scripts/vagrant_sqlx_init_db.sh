#!/usr/bin/env bash
# Why 10.0.2.2 works in vagrant is still not clear
export DATABASE_URL=postgres://postgres:password@10.0.2.2:5432/newsletter
sqlx database create
# This seems to need a migration folder with the table schema
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"