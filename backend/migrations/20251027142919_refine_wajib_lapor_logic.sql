-- Add migration script here
-- This migration refines the wajib lapor feature.
-- It moves the `online_akses` flag to the `klien` table where it belongs,
-- and adds a `metode_lapor` ENUM to the `wajib_lapor_dewasa` table.

-- 1. Add the feature flag to the `klien` table.
--    This column controls whether a client is allowed to use the self-service feature.
ALTER TABLE klien
ADD COLUMN online_akses BOOLEAN NOT NULL DEFAULT FALSE;


-- 2. Drop the incorrect column from the `wajib_lapor_dewasa` table.
ALTER TABLE wajib_lapor_dewasa
DROP COLUMN online_akses;


-- 3. Create a new ENUM type to describe how a report was submitted.
CREATE TYPE metode_lapor_enum AS ENUM ('Mandiri', 'Petugas');


-- 4. Add a new column to `wajib_lapor_dewasa` to store the submission method.
ALTER TABLE wajib_lapor_dewasa
ADD COLUMN metode_lapor metode_lapor_enum NOT NULL;