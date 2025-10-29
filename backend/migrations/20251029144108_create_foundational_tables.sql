-- Add migration script here
-- Migration 1: Create Foundational Tables (Kanwil, Bapas, Users)

-- Reusable function for updating timestamps
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 1. Kanwil Table
CREATE TABLE kanwil (
    id SERIAL PRIMARY KEY,
    nama_kanwil VARCHAR(255) NOT NULL UNIQUE,
    alamat_kanwil TEXT,
    nomor_telepon_kanwil VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON kanwil FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

-- 2. Bapas Table
CREATE TABLE bapas (
    id SERIAL PRIMARY KEY,
    kanwil_id INTEGER NOT NULL REFERENCES kanwil(id) ON DELETE RESTRICT,
    nama_bapas VARCHAR(255) NOT NULL UNIQUE,
    kota_bapas VARCHAR(100) NOT NULL,
    alamat_bapas TEXT,
    nomor_telepon_bapas VARCHAR(50),
    email_bapas VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON bapas FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

-- 3. Users Table
CREATE TYPE user_status_kepegawaian AS ENUM ('Aktif', 'PindahJabatan', 'Pensiun', 'Lainya');
CREATE TYPE user_status_aktif AS ENUM ('Aktif', 'Deaktif');
CREATE TYPE user_role AS ENUM ('Pegawai', 'AdminBapas', 'AdminKanwil', 'SuperAdmin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    nip VARCHAR(50) UNIQUE NOT NULL,
    nama VARCHAR(255) NOT NULL,
    gelar_depan VARCHAR(50),
    gelar_belakang VARCHAR(50),
    pangkat_golongan VARCHAR(100),
    jabatan VARCHAR(255),
    unit_kerja_id INTEGER REFERENCES bapas(id) ON DELETE SET NULL,
    kanwil_id INTEGER REFERENCES kanwil(id) ON DELETE SET NULL,
    status_kepegawaian user_status_kepegawaian NOT NULL,
    email VARCHAR(255) UNIQUE,
    nomor_telepon VARCHAR(50),
    status_aktif user_status_aktif NOT NULL DEFAULT 'Aktif',
    role user_role NOT NULL DEFAULT 'Pegawai',
    password_hash TEXT NOT NULL
);