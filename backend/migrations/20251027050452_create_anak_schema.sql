-- Add migration script here
-- This migration adds the complete and specific set of tables for the 'Anak' client workflow.

-- The reusable audit function `trigger_set_timestamp()` and all base ENUMs
-- are assumed to have been created in a previous migration.


-- 1. The Child Client Reception Table
CREATE TABLE penerimaan_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    
    tanggal_permintaan DATE,
    tanggal_surat_tugas DATE,
    perihal TEXT,
    no_register VARCHAR(100),
    nomor_surat_permintaan VARCHAR(100),
    jenis_permintaan VARCHAR(255),
    nama_instansi VARCHAR(255),
    kelas_instansi VARCHAR(50),
    daerah_instansi VARCHAR(100),
    nama_penjamin_wali VARCHAR(255),
    alamat_penjamin_wali TEXT,
    kelurahan_penjamin_wali VARCHAR(100),
    kecamatan_penjamin_wali VARCHAR(100),
    kota_kabupaten_penjamin_wali VARCHAR(100),

    -- Audit Trail Columns
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();


-- 2. The Child Legal History Table
CREATE TABLE riwayat_hukum_anak (
    id SERIAL PRIMARY KEY,
    penerimaan_anak_id INTEGER NOT NULL UNIQUE REFERENCES penerimaan_anak(id) ON DELETE CASCADE,
    kategori_tindak_pidana VARCHAR(255),
    pasal_tindak_pidana TEXT,
    tanggal_surat_keputusan_pengadilan DATE,
    nomor_surat_keputusan_pengadilan VARCHAR(255),
    pidana_tahun INTEGER,
    pidana_bulan INTEGER,
    pidana_hari INTEGER,
    pertama_ditahan DATE,

    -- Audit Trail Columns
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();


-- 3. The Child Integration Service Table
CREATE TABLE layanan_integrasi_anak (
    id SERIAL PRIMARY KEY,
    penerimaan_anak_id INTEGER NOT NULL REFERENCES penerimaan_anak(id) ON DELETE CASCADE,
    
    nomor_sk VARCHAR(255),
    tanggal_sk DATE,
    nomor_register_integrasi VARCHAR(255),
    masa_bimbingan_awal DATE,
    masa_bimbingan_akhir DATE,
    petugas_layanan_id INTEGER REFERENCES users(id) ON DELETE SET NULL,

    -- Audit Trail Columns
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();


-- 4. The Child Mandatory Reporting Log Table
CREATE TABLE wajib_lapor_anak (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    waktu_lapor TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    photo_path TEXT,
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);


-- 5. The Child Legal Proceedings Table
-- This is the simpler, structured table you requested, replacing the abstract log.
CREATE TABLE proses_hukum_anak (
    id SERIAL PRIMARY KEY,
    penerimaan_anak_id INTEGER NOT NULL REFERENCES penerimaan_anak(id) ON DELETE CASCADE,
    
    jenis_proses_hukum VARCHAR(100), -- e.g., 'Sidang', 'Diversi'
    nomor_register VARCHAR(100),

    -- It's good practice to include a date and audit trails here as well.
    tanggal_proses DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);


-- 6. Indexes for Performance
CREATE INDEX idx_penerimaan_anak_klien_id ON penerimaan_anak(klien_id);
CREATE INDEX idx_layanan_integrasi_anak_penerimaan_id ON layanan_integrasi_anak(penerimaan_anak_id);
CREATE INDEX idx_wajib_lapor_anak_klien_id ON wajib_lapor_anak(klien_id);
CREATE INDEX idx_proses_hukum_anak_penerimaan_id ON proses_hukum_anak(penerimaan_anak_id);