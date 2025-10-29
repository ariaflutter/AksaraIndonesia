-- Add migration script here
-- Migration 3: Create All Dewasa Workflow Tables

CREATE TYPE nama_instansi AS ENUM (
    'Lembaga Pemasyarakatan', 'Rumah Tahanan Negara', 'Balai Pemasyarakatan','Kejaksaan Negeri', 
    'Pengadilan Negeri', 'Kepolisian Resor', 'Kepolisian Sektor', 'Kepolisian Daerah', 
    'Kepolisian Republik Indonesia', 'Pengadilan Tinggi', 'Mahkamah Agung','Lainnya'
);
CREATE TYPE metode_lapor_enum AS ENUM ('Mandiri', 'MandiriDiKantor', 'Petugas');

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
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL
);
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE riwayat_hukum_dewasa (
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
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE layanan_integrasi_dewasa (
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
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE proses_hukum_dewasa (
    id BIGSERIAL PRIMARY KEY,
    penerimaan_dewasa_id INTEGER NOT NULL REFERENCES penerimaan_dewasa(id) ON DELETE CASCADE,
    tipe_event VARCHAR(100) NOT NULL,
    waktu_event TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    detail_event JSONB,
    keterangan TEXT,
    catatan TEXT,
    dicatat_oleh INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT
);

CREATE TABLE wajib_lapor_dewasa (
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