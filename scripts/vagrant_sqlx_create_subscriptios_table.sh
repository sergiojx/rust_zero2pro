#!/usr/bin/env bash
# Why 10.0.2.2 works in vagrant is still not clear
export DATABASE_URL=postgres://postgres:password@10.0.2.2:5432/newsletter

# This will create the migrations folder as well as 
# the related table migration files
# run this from the project directory
sqlx migrate add create_subscriptions_table