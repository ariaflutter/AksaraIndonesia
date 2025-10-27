-- Add migration script here
-- This migration creates the complete relational schema for the 'klien' entity (Version 4.2).

-- Section 1: Custom Data Types (ENUMs)
-- -----------------------------------------------------------------------------
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'tipe_klien') THEN
        CREATE TYPE tipe_klien AS ENUM ('Dewasa', 'Anak');
    END IF;
END$$;
CREATE TYPE tingkat_pendidikan AS ENUM (
    'Tidak Sekolah', 'SD Tidak Lulus', 'SD atau Sederajat', 'SMP atau Sederajat',
    'SMA atau Sederajat', 'D1 atau Sederajat'
);
CREATE TYPE jenis_pekerjaan AS ENUM (
    'Belum/Tidak Bekerja', 'Pelajar/Mahasiswa', 'PNS', 'TNI/Polri', 'Karyawan Swasta',
    'Wiraswasta', 'Petani/Nelayan', 'Lainnya'
);

CREATE TYPE nama_instansi AS ENUM (
    'Lembaga Pemasyarakatan', 'Rumah Tahanan Negara', 'Balai Pemasyarakatan','Kejaksaan Negeri', 'Pengadilan Negeri', 'Kepolisian Resor', 'Kepolisian Sektor', 
    'Kepolisian Daerah', 'Kepolisian Republik Indonesia', 'Pengadilan Tinggi', 'Mahkamah Agung','Lainnya'
);
-- Section 2: Reusable Audit Function
-- -----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Section 3: Core Tables
-- -----------------------------------------------------------------------------

-- Klien Identity Table
CREATE TABLE klien (
    id SERIAL PRIMARY KEY,
    tipe tipe_klien NOT NULL,
    nama TEXT NOT NULL,
    alamat TEXT,
    tempat_lahir VARCHAR(100),
    tanggal_lahir DATE,
    jenis_kelamin VARCHAR(20),
    agama VARCHAR(50),
    pekerjaan jenis_pekerjaan,
    pendidikan_terakhir tingkat_pendidikan,
    bapas_id INTEGER NOT NULL REFERENCES bapas(id) ON DELETE RESTRICT,
    pk_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON klien FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

-- Adult Reception Table
CREATE TABLE penerimaan_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    tanggal_permintaan_lapas DATE,
    tanggal_surat_tugas DATE,
    perihal TEXT,
    no_register_litmas VARCHAR(100),
    nomor_surat_permintaan_lapas VARCHAR(100),
    jenis_permintaan_litmas_lapas VARCHAR(255),
    nama_instansi nama_instansi,
    kelas_instansi VARCHAR(50),
    daerah_instansi VARCHAR(100),
    nama_penjamin VARCHAR(255),
    alamat_penjamin TEXT,
    kelurahan_penjamin TEXT,
    kecamatan_penjamin TEXT,
    kota_kabupaten_penjamin VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

-- Adult Legal History Table
CREATE TABLE riwayat_hukum_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL UNIQUE REFERENCES klien(id) ON DELETE CASCADE,
    kategori_tindak_pidana VARCHAR(255),
    pasal_tindak_pidana TEXT,
    tanggal_surat_keputusan_pengadilan DATE,
    nomor_surat_keputusan_pengadilan VARCHAR(255),
    pidana_tahun INTEGER,
    pidana_bulan INTEGER,
    pidana_hari INTEGER,
    pertama_ditahan DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

-- NEW: Adult Integration Service Table
CREATE TABLE layanan_integrasi_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    
    nomor_sk VARCHAR(255),
    tanggal_sk DATE,
    nomor_register_integrasi VARCHAR(255),
    masa_bimbingan_awal DATE,
    masa_bimbingan_akhir DATE,
    
    -- The user responsible for this specific service.
    petugas_layanan_id INTEGER REFERENCES users(id) ON DELETE SET NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();


-- Section 4: Flexible Event-Log Tables
-- -----------------------------------------------------------------------------

CREATE TABLE proses_hukum_dewasa (
    id BIGSERIAL PRIMARY KEY,
    penerimaan_id INTEGER NOT NULL REFERENCES penerimaan_dewasa(id) ON DELETE CASCADE,
    tipe_event VARCHAR(100) NOT NULL,
    waktu_event TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    detail_event JSONB,
    dicatat_oleh INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE wajib_lapor_dewasa (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    waktu_lapor TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    photo_path TEXT,
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);

-- Section 5: Indexes for Performance
-- -----------------------------------------------------------------------------
CREATE INDEX idx_klien_bapas_id ON klien(bapas_id);
CREATE INDEX idx_klien_pk_id ON klien(pk_id);
CREATE INDEX idx_penerimaan_dewasa_klien_id ON penerimaan_dewasa(klien_id);
CREATE INDEX idx_layanan_integrasi_dewasa_klien_id ON layanan_integrasi_dewasa(klien_id); -- Corrected index
CREATE INDEX idx_proses_hukum_dewasa_penerimaan_id ON proses_hukum_dewasa(penerimaan_dewasa_id); -- Corrected index
CREATE INDEX idx_wajib_lapor_dewasa_klien_id ON wajib_lapor_dewasa(klien_id);
CREATE INDEX idx_riwayat_hukum_dewasa_klien_id ON riwayat_hukum_dewasa(klien_id); -- Corrected index