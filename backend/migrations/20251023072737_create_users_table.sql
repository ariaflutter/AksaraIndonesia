-- Add migration script here
-- Add migration script here
-- This migration creates a detailed 'users' table based on specific requirements.

-- Create ENUM types for fields with a fixed set of values to ensure data integrity.
CREATE TYPE user_status_kepegawaian AS ENUM ('Aktif', 'PindahJabatan', 'Pensiun', 'Lainya');
CREATE TYPE user_status_aktif AS ENUM ('Aktif', 'Deaktif');
CREATE TYPE user_role AS ENUM ('Pegawai', 'AdminBapas', 'SuperAdmin');

-- Create the 'users' table with the detailed columns.
CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    -- Core Identifier (Natural Key)
    nip VARCHAR(50) UNIQUE NOT NULL,

    -- Personal Name Details
    nama VARCHAR(255) NOT NULL,
    gelar_depan VARCHAR(50), -- Optional, so it's nullable
    gelar_belakang VARCHAR(50), -- Optional, so it's nullable

    -- Employment Details (Kepegawaian)
    pangkat_golongan VARCHAR(100),
    jabatan VARCHAR(255),
    
    -- We use a foreign key to the 'bapas' table to represent the 'Unit Kerja'.
    -- This is the correct relational way to link a user to their office.
    unit_kerja_id INTEGER REFERENCES bapas(id) ON DELETE SET NULL,
    
    status_kepegawaian user_status_kepegawaian NOT NULL,

    -- Contact Information
    email VARCHAR(255) UNIQUE, -- Emails should be unique if used for anything important
    nomor_telepon VARCHAR(50),

    -- Application Specific Fields
    -- 'status' is a reserved keyword in SQL, so we rename the column to 'status_aktif'.
    status_aktif user_status_aktif NOT NULL DEFAULT 'Aktif',
    role user_role NOT NULL DEFAULT 'Pegawai',
    
    -- Security
    password_hash TEXT NOT NULL
);

-- Rename the column 'bapas_id' from the original users table design
-- to 'unit_kerja_id' to better match the new field names.
-- Note: This is a comment as we are creating a new table from scratch.
-- In a real scenario of changing an existing table, you would use:
-- ALTER TABLE users RENAME COLUMN bapas_id TO unit_kerja_id;