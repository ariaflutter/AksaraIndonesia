-- Add migration script here
-- Migration: Schema Improvements

-- 1. Membuat ENUM untuk Jenis Kelamin agar data konsisten
CREATE TYPE jenis_kelamin_enum AS ENUM ('Laki-laki', 'Perempuan');

-- Mengubah kolom di tabel klien untuk menggunakan ENUM baru
-- Kita menggunakan USING untuk meng-cast nilai string yang ada ke tipe ENUM baru
ALTER TABLE klien ALTER COLUMN jenis_kelamin TYPE jenis_kelamin_enum
USING (jenis_kelamin::jenis_kelamin_enum);
