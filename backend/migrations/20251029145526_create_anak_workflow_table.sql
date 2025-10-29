-- Add migration script here
-- Migration 4: Create All Anak Workflow Tables

CREATE TABLE penerimaan_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    tanggal_permintaan DATE,
    tanggal_surat_tugas DATE,
    perihal TEXT,
    no_register VARCHAR(100),
    nomor_surat_permintaan VARCHAR(100),
    jenis_permintaan VARCHAR(255),
    nama_instansi nama_instansi,
    kelas_instansi VARCHAR(50),
    daerah_instansi VARCHAR(100),
    nama_penjamin_wali VARCHAR(255),
    alamat_penjamin_wali TEXT,
    kelurahan_penjamin_wali VARCHAR(100),
    kecamatan_penjamin_wali VARCHAR(100),
    kota_kabupaten_penjamin_wali VARCHAR(100),
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE riwayat_hukum_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    kategori_tindak_pidana VARCHAR(255),
    pasal_tindak_pidana TEXT,
    tanggal_surat_keputusan_pengadilan DATE,
    nomor_surat_keputusan_pengadilan VARCHAR(255),
    pidana_tahun INTEGER,
    pidana_bulan INTEGER,
    pidana_hari INTEGER,
    pertama_ditahan DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE layanan_integrasi_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    nomor_sk VARCHAR(255),
    tanggal_sk DATE,
    nomor_register_integrasi VARCHAR(255),
    masa_bimbingan_awal DATE,
    masa_bimbingan_akhir DATE,
    petugas_layanan_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    jenis_bimbingan VARCHAR(255),
    tanggal_surat_pengakhiran DATE,
    nomor_surat_pengakhiran VARCHAR(255),
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE proses_hukum_anak (
    id SERIAL PRIMARY KEY,
    penerimaan_anak_id INTEGER NOT NULL REFERENCES penerimaan_anak(id) ON DELETE CASCADE,
    jenis_proses_hukum VARCHAR(100),
    nomor_register VARCHAR(100),
    tanggal_proses DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE wajib_lapor_anak (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE CASCADE,
    waktu_lapor TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    photo_path TEXT,
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),
    metode_lapor metode_lapor_enum NOT NULL,
    keterangan TEXT,
    catatan TEXT,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);