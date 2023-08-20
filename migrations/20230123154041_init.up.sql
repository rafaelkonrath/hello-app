-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        username VARCHAR NOT NULL unique,
        date_of_birth VARCHAR(10) 
    );