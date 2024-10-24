-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);