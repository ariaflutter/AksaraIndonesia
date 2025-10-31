-- Add migration script here
ALTER TABLE klien ADD COLUMN pin_klien_hash TEXT;
-- migrations/YYYYMMDDHHMMSS_remove_waktu_lapor.sql

-- Hapus kolom waktu_lapor_dewasa karena created_at sudah mencukupi
ALTER TABLE wajib_lapor_dewasa
DROP COLUMN waktu_lapor_dewasa;

-- Hapus kolom waktu_lapor_anak
ALTER TABLE wajib_lapor_anak
DROP COLUMN waktu_lapor_anak;