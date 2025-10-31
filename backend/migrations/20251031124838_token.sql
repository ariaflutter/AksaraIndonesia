-- Add migration script here
-- migrations/YYYYMMDDHHMMSS_add_api_key_to_users.sql

ALTER TABLE users
ADD COLUMN api_key_hash TEXT UNIQUE; -- Harus UNIQUE untuk mencegah duplikasi

-- Opsional: Buat indeks untuk pencarian cepat berdasarkan hash
CREATE INDEX idx_users_api_key_hash ON users(api_key_hash);