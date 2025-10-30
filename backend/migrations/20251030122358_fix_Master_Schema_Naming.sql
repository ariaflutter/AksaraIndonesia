-- Add migration script here
-- Add migration script here
-- =============================================================================
-- SKEMA MASTER AKSARA V1
--
-- Skrip ini mendefinisikan seluruh skema database dari awal,
-- sudah mencakup struktur, relasi, soft delete, trigger, dan indeks performa.
-- =============================================================================


-- =============================================================================
-- LANGKAH 1: SETUP AWAL (FUNGSI & EKSTENSI)
-- =============================================================================

-- Fungsi reusable untuk memperbarui kolom 'updated_at' secara otomatis
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Ekstensi untuk pencarian teks cepat (LIKE '%...%')
CREATE EXTENSION IF NOT EXISTS pg_trgm;


-- =============================================================================
-- LANGKAH 2: DEFINISI TIPE DATA KUSTOM (ENUMs)
-- Mendefinisikan semua ENUM di satu tempat untuk kejelasan.
-- =============================================================================

CREATE TYPE user_status_kepegawaian_enum AS ENUM ('Aktif', 'Pindah Jabatan', 'Pensiun', 'Lainnya');
CREATE TYPE user_status_aktif_enum AS ENUM ('Aktif', 'Deaktif');
CREATE TYPE user_role_enum AS ENUM ('Pegawai', 'AdminBapas', 'AdminKanwil', 'SuperAdmin');
CREATE TYPE tipe_klien_enum AS ENUM ('Dewasa', 'Anak');
CREATE TYPE tingkat_pendidikan_enum AS ENUM ('Tidak Sekolah', 'SD Tidak Lulus', 'SD atau Sederajat', 'SMP atau Sederajat', 'SMA atau Sederajat', 'D1 atau Sederajat', 'D2 atau Sederajat', 'D3 atau Sederajat', 'S1 atau Sederajat', 'S2 atau Sederajat', 'S3 atau Sederajat');
CREATE TYPE jenis_pekerjaan_enum AS ENUM (
'Belum/Tidak Bekerja',
'Pegawai Negeri Sipil',
'Tentara Nasional Indonesia',
'Kepolisian RI',
'Karyawan BUMN',
'Karyawan BUMD',
'Anggota DPR-RI',
'Anggota DPD',
'Anggota BPK',
'Presiden',
'Wakil Presiden',
'Anggota Mahkamah Konstitusi',
'Anggota Kabinet/Kementerian',
'Duta Besar',
'Gubernur',
'Wakil Gubernur',
'Bupati',
'Wakil Bupati',
'Walikota',
'Wakil Walikota',
'Anggota DPRD Provinsi',
'Anggota DPRD Kabupaten/Kota',
'Pengacara',
'Notaris',
'Peneliti',
'Perangkat Desa',
'Kepala Desa',
'Dosen',
'Guru',
'Perdagangan',
'Industri',
'Konstruksi',
'Transportasi',
'Karyawan Swasta',
'Karyawan Honorer',
'Buruh Harian Lepas',
'Pembantu Rumah Tangga',
'Tukang Cukur',
'Tukang Listrik',
'Tukang Batu',
'Tukang Kayu',
'Tukang Sol Sepatu',
'Tukang Las/Pandai Besi',
'Tukang Jahit',
'Tukang Gigi',
'Penata Rias',
'Penata Busana',
'Penata Rambut',
'Mekanik',
'Seniman',
'Tabib',
'Paraji',
'Perancang Busana',
'Penterjemah',
'Wartawan',
'Juru Masak',
'Promotor Acara',
'Pilot',
'Arsitek',
'Akuntan',
'Konsultan',
'Penyiar Televisi',
'Penyiar Radio',
'Pelaut',
'Sopir',
'Pialang',
'Paranormal',
'Pedagang',
'Wiraswasta',
'Petani/Pekebun',
'Peternak',
'Buruh Tani/Perkebunan',
'Buruh Peternakan',
'Nelayan/Perikanan',
'Buruh Nelayan/Perikanan',
'Imam Mesjid',
'Pendeta',
'Pastor',
'Ustadz/Mubaligh',
'Biarawati',
'Pelajar/Mahasiswa',
'Dokter',
'Bidan',
'Perawat',
'Apoteker',
'Psikiater/Psikolog',
'Pensiunan',
'Mengurus Rumah Tangga',
'Lainnya'
);
CREATE TYPE jenis_kelamin_enum AS ENUM ('Laki-laki', 'Perempuan');
CREATE TYPE nama_instansi_enum AS ENUM ('Lembaga Pemasyarakatan', 'Rumah Tahanan Negara', 'Balai Pemasyarakatan','Kejaksaan Negeri', 'Pengadilan Negeri', 'Kepolisian Resor', 'Kepolisian Sektor', 'Kepolisian Daerah', 'Kepolisian Republik Indonesia', 'Pengadilan Tinggi', 'Mahkamah Agung','Lainnya');
CREATE TYPE metode_lapor_enum AS ENUM ('Online', 'Self-Service', 'Petugas');
CREATE TYPE kewarganegaraan_enum AS ENUM ('WNI', 'WNA'); -- Example values

  


-- =============================================================================
-- LANGKAH 3: PEMBUATAN TABEL-TABEL UTAMA
-- =============================================================================

-- 1. Kanwil Table
CREATE TABLE kanwil (
    id SERIAL PRIMARY KEY,
    nama_kanwil VARCHAR(255) NOT NULL UNIQUE,
    alamat_kanwil TEXT,
    nomor_telepon_kanwil VARCHAR(50),
    email_kanwil VARCHAR(255) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);


-- 2. Bapas Table
CREATE TABLE bapas (
    id SERIAL PRIMARY KEY,
    kanwil_id INTEGER NOT NULL REFERENCES kanwil(id) ON DELETE RESTRICT,
    nama_bapas VARCHAR(255) NOT NULL UNIQUE,
    kota_bapas VARCHAR(100) NOT NULL,
    alamat_bapas TEXT,
    nomor_telepon_bapas VARCHAR(50),
    email_bapas VARCHAR(255) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- 3. Users Table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    nip_user VARCHAR(50) NOT NULL UNIQUE, -- NIP unik secara absolut
    nama_user VARCHAR(255) NOT NULL,
    gelar_depan_user VARCHAR(50),
    gelar_belakang_user VARCHAR(50),
    pangkat_golongan_user VARCHAR(100),
    jabatan_user VARCHAR(255),
    bapas_id INTEGER REFERENCES bapas(id) ON DELETE SET NULL, -- Sudah diganti nama demi konsistensi
    kanwil_id INTEGER REFERENCES kanwil(id) ON DELETE SET NULL, -- Akan diisi oleh trigger
    status_kepegawaian_user user_status_kepegawaian_enum NOT NULL,
    email_user VARCHAR(255), -- Keunikan diatur oleh partial index di bawah
    nomor_telepon_user VARCHAR(50),
    status_aktif_user user_status_aktif_enum NOT NULL DEFAULT 'Aktif',
    role_user user_role_enum NOT NULL DEFAULT 'Pegawai',
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

-- 4. Klien Table
CREATE TABLE klien (
    id SERIAL PRIMARY KEY,
    tipe_klien tipe_klien_enum NOT NULL,
    nama_klien TEXT NOT NULL,
    alamat_klien TEXT,
    tempat_lahir_klien VARCHAR(100),
    tanggal_lahir_klien DATE,
    jenis_kelamin_klien jenis_kelamin_enum, -- Menggunakan ENUM
    agama_klien VARCHAR(50),
    pekerjaan_klien jenis_pekerjaan_enum,
    pendidikan_terakhir_klien tingkat_pendidikan_enum,
    bapas_id INTEGER NOT NULL REFERENCES bapas(id) ON DELETE RESTRICT,
    pk_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    kanwil_id INTEGER REFERENCES kanwil(id) ON DELETE SET NULL,
    online_akses_klien BOOLEAN NOT NULL DEFAULT FALSE,
    pengulangan_klien BOOLEAN NOT NULL DEFAULT FALSE,
    kewarganegaraan_klien kewarganegaraan_enum,
    negara_asal_klien VARCHAR(100),
    suku_klien VARCHAR(100),
    keterangan_klien TEXT,
    catatan_klien TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

--Tabel Dewasa--
CREATE TABLE penerimaan_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    tanggal_permintaan_lapas_dewasa DATE,
    tanggal_surat_tugas_dewasa DATE,
    perihal_dewasa TEXT,
    no_register_litmas_dewasa VARCHAR(100),
    nomor_surat_permintaan_lapas_dewasa VARCHAR(100),
    jenis_permintaan_litmas_lapas_dewasa VARCHAR(255),
    nama_instansi_dewasa nama_instansi_enum,
    kelas_instansi_dewasa VARCHAR(50),
    daerah_instansi_dewasa VARCHAR(100),
    nama_penjamin_dewasa VARCHAR(255),
    alamat_penjamin_dewasa TEXT,
    kelurahan_penjamin_dewasa TEXT,
    kecamatan_penjamin_dewasa TEXT,
    kota_kabupaten_penjamin_dewasa VARCHAR(100),
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE riwayat_hukum_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    kategori_tindak_pidana_dewasa VARCHAR(255),
    pasal_tindak_pidana_dewasa TEXT,
    tanggal_surat_keputusan_pengadilan_dewasa DATE,
    nomor_surat_keputusan_pengadilan_dewasa VARCHAR(255),
    pidana_tahun_dewasa INTEGER,
    pidana_bulan_dewasa INTEGER,
    pidana_hari_dewasa INTEGER,
    pertama_ditahan_dewasa DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE layanan_integrasi_dewasa (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    nomor_sk_dewasa VARCHAR(255),
    tanggal_sk_integrasi_dewasa DATE,
    nomor_register_integrasi_dewasa VARCHAR(255),
    masa_bimbingan_awal_dewasa DATE,
    masa_bimbingan_akhir_dewasa DATE,
    petugas_layanan_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    jenis_bimbingan_dewasa VARCHAR(255),
    tanggal_surat_pengakhiran_dewasa DATE,
    nomor_surat_pengakhiran_dewasa VARCHAR(255),
    pengakhiran_dewasa BOOLEAN DEFAULT FALSE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE wajib_lapor_dewasa (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    waktu_lapor_dewasa TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    photo_path_dewasa TEXT,
    latitude_dewasa DECIMAL(9, 6),
    longitude_dewasa DECIMAL(9, 6),
    metode_lapor_dewasa metode_lapor_enum NOT NULL,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ -- Menambahkan soft delete di sini
);

CREATE TABLE proses_hukum_dewasa (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER REFERENCES klien(id) ON DELETE RESTRICT,
    penerimaan_dewasa_id INTEGER NOT NULL REFERENCES penerimaan_dewasa(id) ON DELETE RESTRICT,
    jenis_proses_hukum_dewasa VARCHAR(100),
    nomor_register_proses_hukum_dewasa VARCHAR(100),
    tanggal_proses_dewasa DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

-- Tabel Anak --

CREATE TABLE penerimaan_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    tanggal_permintaan_lapas_anak DATE,
    tanggal_surat_tugas_anak DATE,
    perihal_anak TEXT,
    no_register_litmas_anak VARCHAR(100),
    nomor_surat_permintaan_lapas_anak VARCHAR(100),
    jenis_permintaan_litmas_lapas_anak VARCHAR(255),
    nama_instansi_anak nama_instansi_enum,
    kelas_instansi_anak VARCHAR(50),
    daerah_instansi_anak VARCHAR(100),
    nama_penjamin_anak VARCHAR(255),
    alamat_penjamin_anak TEXT,
    kelurahan_penjamin_anak TEXT,
    kecamatan_penjamin_anak TEXT,
    kota_kabupaten_penjamin_anak VARCHAR(100),
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE riwayat_hukum_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    kategori_tindak_pidana_anak VARCHAR(255),
    pasal_tindak_pidana_anak TEXT,
    tanggal_surat_keputusan_pengadilan_anak DATE,
    nomor_surat_keputusan_pengadilan_anak VARCHAR(255),
    pidana_tahun_anak INTEGER,
    pidana_bulan_anak INTEGER,
    pidana_hari_anak INTEGER,
    pertama_ditahan_anak DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE layanan_integrasi_anak (
    id SERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    nomor_sk_anak VARCHAR(255),
    tanggal_sk_integrasi_anak DATE,
    nomor_register_integrasi_anak VARCHAR(255),
    masa_bimbingan_awal_anak DATE,
    masa_bimbingan_akhir_anak DATE,
    petugas_layanan_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    jenis_bimbingan_anak VARCHAR(255),
    tanggal_surat_pengakhiran_anak DATE,
    nomor_surat_pengakhiran_anak VARCHAR(255),
    pengakhiran_anak BOOLEAN DEFAULT FALSE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE wajib_lapor_anak (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER NOT NULL REFERENCES klien(id) ON DELETE RESTRICT,
    waktu_lapor_anak TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    photo_path_anak TEXT,
    latitude_anak DECIMAL(9, 6),
    longitude_anak DECIMAL(9, 6),
    metode_lapor_anak metode_lapor_enum NOT NULL,
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ -- Menambahkan soft delete di sini
);

CREATE TABLE proses_hukum_anak (
    id BIGSERIAL PRIMARY KEY,
    klien_id INTEGER REFERENCES klien(id) ON DELETE RESTRICT,
    penerimaan_anak_id INTEGER NOT NULL REFERENCES penerimaan_anak(id) ON DELETE RESTRICT,
    jenis_proses_hukum_anak VARCHAR(100),
    nomor_register_proses_hukum_anak VARCHAR(100),
    tanggal_proses_anak DATE,
    keterangan TEXT,
    catatan TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    updated_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ
);


-- =============================================================================
-- LANGKAH 5: DEFINISI PERILAKU OTOMATIS (TRIGGERS)
-- Menambahkan "kecerdasan" ke database untuk menjaga konsistensi dan integritas data.
-- =============================================================================

CREATE OR REPLACE FUNCTION cascade_soft_delete_from_klien()
RETURNS TRIGGER AS $$
BEGIN
    -- Trigger ini berjalan SETELAH klien di-update.
    -- Ia akan menyinkronkan status 'deleted_at' ke semua tabel anak.
    
    -- Update tabel workflow Dewasa
    UPDATE penerimaan_dewasa SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE riwayat_hukum_dewasa SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE layanan_integrasi_dewasa SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE wajib_lapor_dewasa SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE proses_hukum_dewasa SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;

      -- Update tabel workflow Anak
    UPDATE penerimaan_anak SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE riwayat_hukum_anak SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE layanan_integrasi_anak SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE wajib_lapor_anak SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;
    UPDATE proses_hukum_anak SET deleted_at = NEW.deleted_at WHERE klien_id = NEW.id;

        RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_cascade_soft_delete_klien
    AFTER UPDATE ON klien
    FOR EACH ROW
    -- Optimasi: Trigger HANYA berjalan jika kolom 'deleted_at' yang berubah.
    WHEN (OLD.deleted_at IS DISTINCT FROM NEW.deleted_at)
    EXECUTE FUNCTION cascade_soft_delete_from_klien();

CREATE TRIGGER set_timestamp BEFORE UPDATE ON kanwil FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON bapas FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON klien FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON proses_hukum_dewasa FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON penerimaan_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON riwayat_hukum_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON layanan_integrasi_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp BEFORE UPDATE ON proses_hukum_anak FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp();





-- Indeks Dasar (Foreign Keys & Audit)
CREATE INDEX idx_bapas_kanwil_id ON bapas(kanwil_id);
CREATE INDEX idx_users_bapas_id ON users(bapas_id);
CREATE INDEX idx_klien_bapas_id ON klien(bapas_id);
CREATE INDEX idx_klien_pk_id ON klien(pk_id);
CREATE INDEX idx_klien_kanwil_id ON klien(kanwil_id);
CREATE INDEX idx_klien_created_by ON klien(created_by);
CREATE INDEX idx_penerimaan_dewasa_klien_id ON penerimaan_dewasa(klien_id);
CREATE INDEX idx_riwayat_hukum_dewasa_klien_id ON riwayat_hukum_dewasa(klien_id);
CREATE INDEX idx_layanan_integrasi_dewasa_klien_id ON layanan_integrasi_dewasa(klien_id);
CREATE INDEX idx_wajib_lapor_dewasa_klien_id ON wajib_lapor_dewasa(klien_id);
CREATE INDEX idx_proses_hukum_dewasa_penerimaan_id ON proses_hukum_dewasa(penerimaan_dewasa_id);
CREATE INDEX idx_penerimaan_anak_klien_id ON penerimaan_anak(klien_id);
CREATE INDEX idx_riwayat_hukum_anak_klien_id ON riwayat_hukum_anak(klien_id);
CREATE INDEX idx_layanan_integrasi_anak_klien_id ON layanan_integrasi_anak(klien_id);
CREATE INDEX idx_wajib_lapor_anak_klien_id ON wajib_lapor_anak(klien_id);
CREATE INDEX idx_proses_hukum_anak_penerimaan_id ON proses_hukum_anak(penerimaan_anak_id);

-- Indeks Komposit (UI, Filter, Urutan)
CREATE INDEX idx_klien_pk_id_created_at_desc ON klien(pk_id, created_at DESC);
CREATE INDEX idx_klien_bapas_id_created_at_desc ON klien(bapas_id, created_at DESC);
CREATE INDEX idx_klien_bapas_id_tipe ON klien(bapas_id, tipe_klien);
CREATE INDEX idx_users_bapas_id_role ON users(bapas_id, role_user);
CREATE INDEX idx_wajib_lapor_dewasa_klien_id_waktu_lapor_desc ON wajib_lapor_dewasa(klien_id, waktu_lapor_dewasa DESC);
CREATE INDEX idx_wajib_lapor_anak_klien_id_waktu_lapor_desc ON wajib_lapor_anak(klien_id, waktu_lapor_anak DESC);

-- Indeks Pencarian & Analitik (Teks & Waktu)
CREATE INDEX idx_klien_nama_trgm ON klien USING GIN (nama_klien gin_trgm_ops);
CREATE INDEX idx_users_nip_trgm ON users USING GIN (nip_user gin_trgm_ops);
CREATE INDEX idx_users_nama_trgm ON users USING GIN (nama_user gin_trgm_ops);
CREATE INDEX idx_klien_created_at ON klien(created_at);
CREATE INDEX idx_layanan_integrasi_dewasa_masa_bimbingan ON layanan_integrasi_dewasa(masa_bimbingan_awal_dewasa, masa_bimbingan_akhir_dewasa);
CREATE INDEX idx_layanan_integrasi_anak_masa_bimbingan ON layanan_integrasi_anak(masa_bimbingan_awal_anak, masa_bimbingan_akhir_anak);
CREATE INDEX idx_klien_kanwil_bapas_tipe ON klien(kanwil_id, bapas_id, tipe_klien);
CREATE INDEX idx_proses_hukum_dewasa_klien_id ON proses_hukum_dewasa(klien_id);
CREATE INDEX idx_proses_hukum_anak_klien_id ON proses_hukum_anak(klien_id);

-- 1. Trigger untuk sinkronisasi kanwil_id di tabel users
CREATE OR REPLACE FUNCTION sync_user_kanwil_id_from_bapas()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.bapas_id IS NOT NULL THEN
        SELECT kanwil_id INTO NEW.kanwil_id FROM bapas WHERE id = NEW.bapas_id;
    ELSE
        NEW.kanwil_id = NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- (Tambahkan ini di dalam LANGKAH 5 di migrasi Anda)

-- 3. Trigger untuk sinkronisasi bapas_id dan kanwil_id di tabel KLIEN
--    berdasarkan pk_id yang ditugaskan.
CREATE OR REPLACE FUNCTION sync_klien_location_from_pk()
RETURNS TRIGGER AS $$
DECLARE
    pk_bapas_id INTEGER;
    pk_kanwil_id INTEGER;
BEGIN
    -- Cek apakah pk_id diisi. Trigger ini hanya berjalan jika ada PK.
    IF NEW.pk_id IS NOT NULL THEN
        -- Lakukan lookup ke tabel users untuk mendapatkan bapas_id dan kanwil_id
        -- dari PK yang bersangkutan.
        SELECT
            u.bapas_id,
            u.kanwil_id -- Kita bisa ambil dari users karena sudah disinkronkan oleh trigger lain
        INTO
            pk_bapas_id,
            pk_kanwil_id
        FROM
            users u
        WHERE
            u.id = NEW.pk_id;

        -- Jika PK ditemukan, set bapas_id dan kanwil_id pada baris klien yang baru.
        -- Ini akan menimpa nilai apapun yang coba dimasukkan oleh aplikasi.
        IF FOUND THEN
            NEW.bapas_id = pk_bapas_id;
            NEW.kanwil_id = pk_kanwil_id;
        ELSE
            -- Jika pk_id yang diberikan tidak ada di tabel users,
            -- proses INSERT/UPDATE akan gagal karena foreign key constraint.
            -- Namun, sebagai pengaman, kita bisa melempar error yang lebih jelas.
            RAISE EXCEPTION 'PK dengan id % tidak ditemukan.', NEW.pk_id;
        END IF;
    END IF;

    -- Kembalikan baris klien yang sudah dimodifikasi untuk disimpan.
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION sync_klien_location_from_pk()
RETURNS TRIGGER AS $$
BEGIN
  IF TG_OP = 'UPDATE' AND NEW.pk_id IS NOT DISTINCT FROM OLD.pk_id THEN
    RETURN NEW;  -- skip trigger if pk_id didn’t change
  END IF;

  -- your existing logic for syncing location here
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 4. Trigger untuk mengisi klien_id di proses_hukum_dewasa secara otomatis
CREATE OR REPLACE FUNCTION sync_proses_hukum_dewasa_klien_id()
RETURNS TRIGGER AS $$
BEGIN
    -- Ambil klien_id dari tabel 'penerimaan_dewasa' dan masukkan
    -- ke dalam baris 'proses_hukum_dewasa' yang baru.
    SELECT klien_id INTO NEW.klien_id
    FROM penerimaan_dewasa
    WHERE id = NEW.penerimaan_dewasa_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_sync_proses_hukum_dewasa_klien_id
    BEFORE INSERT ON proses_hukum_dewasa
    FOR EACH ROW
    EXECUTE FUNCTION sync_proses_hukum_dewasa_klien_id();

-- 5. Trigger untuk mengisi klien_id di proses_hukum_anak secara otomatis
CREATE OR REPLACE FUNCTION sync_proses_hukum_anak_klien_id()
RETURNS TRIGGER AS $$
BEGIN
    SELECT klien_id INTO NEW.klien_id
    FROM penerimaan_anak
    WHERE id = NEW.penerimaan_anak_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_sync_proses_hukum_anak_klien_id
    BEFORE INSERT ON proses_hukum_anak
    FOR EACH ROW
    EXECUTE FUNCTION sync_proses_hukum_anak_klien_id();

CREATE TRIGGER trg_sync_user_kanwil
    BEFORE INSERT OR UPDATE OF bapas_id ON users
    FOR EACH ROW
    EXECUTE FUNCTION sync_user_kanwil_id_from_bapas();