-- Add migration script here
-- Migration: Add a standard UNIQUE constraint to the nip column in the users table

-- Menambahkan constraint UNIQUE ke kolom 'nip'.
-- 'users_nip_key' adalah nama standar untuk constraint ini.
ALTER TABLE users ADD CONSTRAINT users_nip_key UNIQUE (nip);