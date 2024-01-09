-- Add migration script here

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name text NOT NULL UNIQUE,
    email text NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    password text NOT NULL 
);

CREATE TABLE chats (
    id UUID PRIMARY KEY,
    name text NOT NULL,
    creator UUID NOT NULL REFERENCES users(id),
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    password text NOT NULL
)