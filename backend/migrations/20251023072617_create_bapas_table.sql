-- Add migration script here
-- This migration creates the initial 'bapas' table.
-- This table will store a list of all Bapas offices.

CREATE TABLE bapas (
    -- 'SERIAL' is an auto-incrementing integer (1, 2, 3...).
    -- 'PRIMARY KEY' means this is the unique identifier for each row.
    id SERIAL PRIMARY KEY,

    -- 'VARCHAR(255)' is a text field with a max length.
    -- 'NOT NULL' means this field cannot be empty.
    -- 'UNIQUE' means no two Bapas offices can have the same name.
    nama_bapas VARCHAR(255) NOT NULL UNIQUE,

    -- 'VARCHAR(100)' for the city name.
    kota VARCHAR(100) NOT NULL,

    -- 'TEXT' is for longer pieces of text with no specific length limit.
    alamat TEXT,

    -- 'VARCHAR(50)' is suitable for phone numbers.
    nomor_telepon_bapas VARCHAR(50),

    -- An email address for the office.
    email VARCHAR(255),

    -- kanwil
    kanwil VARCHAR(255)
);