-- Add migration script here
-- This migration adds an 'online_akses' column to the 'wajib_lapor_dewasa' table
-- to distinguish between online and in-person check-ins.

ALTER TABLE wajib_lapor_dewasa
ADD COLUMN online_akses BOOLEAN NOT NULL DEFAULT FALSE;