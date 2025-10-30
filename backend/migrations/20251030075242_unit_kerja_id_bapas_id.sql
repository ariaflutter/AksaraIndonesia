-- Add migration script here
-- Migration: Rename 'unit_kerja_id' in 'users' to 'bapas_id' for consistency

-- 1. Ganti nama kolomnya
ALTER TABLE users RENAME COLUMN unit_kerja_id TO bapas_id;

-- 2. Ganti nama indeks yang merujuk pada kolom tersebut
ALTER INDEX idx_users_unit_kerja_id RENAME TO idx_users_bapas_id;
ALTER INDEX idx_users_unit_kerja_id_role RENAME TO idx_users_bapas_id_role;

-- 3. Perbarui fungsi trigger yang menggunakan nama kolom lama
-- (Definisi ulang fungsi dengan nama kolom yang baru)
CREATE OR REPLACE FUNCTION sync_user_kanwil_id_from_bapas()
RETURNS TRIGGER AS $$
BEGIN
    -- Cek bapas_id (nama baru)
    IF NEW.bapas_id IS NOT NULL THEN
        -- Ambil kanwil_id dari tabel bapas berdasarkan bapas_id (nama baru)
        SELECT kanwil_id INTO NEW.kanwil_id
        FROM bapas
        WHERE id = NEW.bapas_id;
    ELSE
        NEW.kanwil_id = NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;