-- Add migration script here
-- Migration: Create a trigger to automatically sync user's kanwil_id from their bapas_id (unit_kerja_id)

-- =============================================================================
-- LANGKAH 1: BUAT FUNGSI TRIGGER
-- Fungsi ini berisi logika yang akan dijalankan: "Ambil kanwil_id dari tabel bapas".
-- =============================================================================

CREATE OR REPLACE FUNCTION sync_user_kanwil_id_from_bapas()
RETURNS TRIGGER AS $$
BEGIN
    -- Cek apakah unit_kerja_id (yang merupakan bapas_id) diisi
    IF NEW.unit_kerja_id IS NOT NULL THEN
        -- Lakukan query ke tabel 'bapas' untuk mencari kanwil_id
        -- yang sesuai, lalu simpan hasilnya ke kolom 'kanwil_id'
        -- di baris user yang sedang diproses.
        -- 'NEW' adalah variabel spesial yang merujuk pada baris
        -- yang akan di-INSERT atau di-UPDATE.
        SELECT kanwil_id INTO NEW.kanwil_id
        FROM bapas
        WHERE id = NEW.unit_kerja_id;
    ELSE
        -- Jika unit_kerja_id dihapus (menjadi NULL),
        -- maka kanwil_id juga harus di-set NULL.
        NEW.kanwil_id = NULL;
    END IF;

    -- Kembalikan baris yang sudah dimodifikasi agar proses INSERT/UPDATE bisa lanjut.
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- =============================================================================
-- LANGKAH 2: PASANG TRIGGER KE TABEL 'users'
-- Trigger ini memberitahu database KAPAN dan BAGAIMANA fungsi di atas harus dijalankan.
-- =============================================================================

-- Hapus trigger lama jika ada (membuat skrip bisa dijalankan ulang)
DROP TRIGGER IF EXISTS trg_sync_user_kanwil_id ON users;

-- Buat trigger baru
CREATE TRIGGER trg_sync_user_kanwil_id
    -- Jalankan SEBELUM data benar-benar disimpan
    BEFORE INSERT OR UPDATE ON users
    -- Jalankan untuk setiap baris yang terpengaruh
    FOR EACH ROW
    -- Jalankan fungsi yang sudah kita buat di atas
    EXECUTE FUNCTION sync_user_kanwil_id_from_bapas();

-- =============================================================================