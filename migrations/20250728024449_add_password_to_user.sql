-- Add password column to users table
ALTER TABLE users ADD COLUMN password_hash TEXT NOT NULL DEFAULT '';

