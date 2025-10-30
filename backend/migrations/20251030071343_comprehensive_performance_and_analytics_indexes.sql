-- Migration: Add Comprehensive Performance and Analytics Indexes

-- =============================================================================
-- LANGKAH 1: AKTIFKAN EKSTENSI YANG DIPERLUKAN
-- =============================================================================
-- pg_trgm digunakan untuk pencarian teks cepat (LIKE '%...%')
CREATE EXTENSION IF NOT EXISTS pg_trgm;


-- =============================================================================
-- LANGKAH 2: INDEKS DASAR (FOREIGN KEY & KOLOM AUDIT)
-- Ini adalah fondasi performa, mempercepat semua operasi JOIN.
-- =============================================================================

-- Tabel Bapas
CREATE INDEX IF NOT EXISTS idx_bapas_kanwil_id ON bapas(kanwil_id);

-- Tabel Users
CREATE INDEX IF NOT EXISTS idx_users_unit_kerja_id ON users(unit_kerja_id);

-- Tabel Klien
CREATE INDEX IF NOT EXISTS idx_klien_bapas_id ON klien(bapas_id);
CREATE INDEX IF NOT EXISTS idx_klien_pk_id ON klien(pk_id);
CREATE INDEX IF NOT EXISTS idx_klien_kanwil_id ON klien(kanwil_id);
CREATE INDEX IF NOT EXISTS idx_klien_created_by ON klien(created_by);
CREATE INDEX IF NOT EXISTS idx_klien_updated_by ON klien(updated_by);

-- Tabel-tabel Workflow (Dewasa)
CREATE INDEX IF NOT EXISTS idx_penerimaan_dewasa_klien_id ON penerimaan_dewasa(klien_id);
CREATE INDEX IF NOT EXISTS idx_penerimaan_dewasa_created_by ON penerimaan_dewasa(created_by);
CREATE INDEX IF NOT EXISTS idx_penerimaan_dewasa_updated_by ON penerimaan_dewasa(updated_by);

CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_dewasa_klien_id ON riwayat_hukum_dewasa(klien_id);
CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_dewasa_created_by ON riwayat_hukum_dewasa(created_by);
CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_dewasa_updated_by ON riwayat_hukum_dewasa(updated_by);

CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_dewasa_klien_id ON layanan_integrasi_dewasa(klien_id);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_dewasa_petugas_id ON layanan_integrasi_dewasa(petugas_layanan_id);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_dewasa_created_by ON layanan_integrasi_dewasa(created_by);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_dewasa_updated_by ON layanan_integrasi_dewasa(updated_by);

CREATE INDEX IF NOT EXISTS idx_wajib_lapor_dewasa_klien_id ON wajib_lapor_dewasa(klien_id);
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_dewasa_created_by ON wajib_lapor_dewasa(created_by);

-- Tabel-tabel Workflow (Anak)
CREATE INDEX IF NOT EXISTS idx_penerimaan_anak_klien_id ON penerimaan_anak(klien_id);
CREATE INDEX IF NOT EXISTS idx_penerimaan_anak_created_by ON penerimaan_anak(created_by);
CREATE INDEX IF NOT EXISTS idx_penerimaan_anak_updated_by ON penerimaan_anak(updated_by);

CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_anak_klien_id ON riwayat_hukum_anak(klien_id);
CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_anak_created_by ON riwayat_hukum_anak(created_by);
CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_anak_updated_by ON riwayat_hukum_anak(updated_by);

CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_anak_klien_id ON layanan_integrasi_anak(klien_id);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_anak_petugas_id ON layanan_integrasi_anak(petugas_layanan_id);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_anak_created_by ON layanan_integrasi_anak(created_by);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_anak_updated_by ON layanan_integrasi_anak(updated_by);

CREATE INDEX IF NOT EXISTS idx_wajib_lapor_anak_klien_id ON wajib_lapor_anak(klien_id);
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_anak_created_by ON wajib_lapor_anak(created_by);


-- =============================================================================
-- LANGKAH 3: INDEKS KOMPOSIT UNTUK UI, FILTER, DAN URUTAN
-- Ini adalah optimasi untuk query-query paling umum di aplikasi Anda.
-- =============================================================================

-- Mempercepat query list klien milik PK atau Bapas, diurutkan dari terbaru (sangat umum!)
CREATE INDEX IF NOT EXISTS idx_klien_pk_id_created_at_desc ON klien(pk_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_klien_bapas_id_created_at_desc ON klien(bapas_id, created_at DESC);

-- Mempercepat filter klien di halaman daftar
CREATE INDEX IF NOT EXISTS idx_klien_bapas_id_tipe ON klien(bapas_id, tipe);
CREATE INDEX IF NOT EXISTS idx_klien_bapas_id_pengulangan ON klien(bapas_id, pengulangan);

-- Mempercepat list user (PK) di dalam Bapas
CREATE INDEX IF NOT EXISTS idx_users_unit_kerja_id_role ON users(unit_kerja_id, role);

-- Mempercepat query list wajib lapor milik seorang klien, diurutkan dari terbaru
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_dewasa_klien_id_waktu_lapor_desc ON wajib_lapor_dewasa(klien_id, waktu_lapor DESC);
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_anak_klien_id_waktu_lapor_desc ON wajib_lapor_anak(klien_id, waktu_lapor DESC);


-- =============================================================================
-- LANGKAH 4: INDEKS UNTUK PENCARIAN & ANALITIK (RENTANG WAKTU)
-- =============================================================================

-- Pencarian teks cepat (case-insensitive LIKE '%...%') pada kolom-kolom penting
CREATE INDEX IF NOT EXISTS idx_klien_nama_trgm ON klien USING GIN (nama gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_users_nip_trgm ON users USING GIN (nip gin_trgm_ops);
CREATE INDEX IF NOT EXISTS idx_users_nama_trgm ON users USING GIN (nama gin_trgm_ops);

-- Mempercepat laporan berdasarkan waktu registrasi klien
CREATE INDEX IF NOT EXISTS idx_klien_created_at ON klien(created_at);

-- Mempercepat notifikasi & laporan untuk layanan integrasi yang akan berakhir
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_dewasa_masa_bimbingan ON layanan_integrasi_dewasa(masa_bimbingan_awal, masa_bimbingan_akhir);
CREATE INDEX IF NOT EXISTS idx_layanan_integrasi_anak_masa_bimbingan ON layanan_integrasi_anak(masa_bimbingan_awal, masa_bimbingan_akhir);

-- Mempercepat laporan wajib lapor berdasarkan periode waktu
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_dewasa_waktu_lapor ON wajib_lapor_dewasa(waktu_lapor);
CREATE INDEX IF NOT EXISTS idx_wajib_lapor_anak_waktu_lapor ON wajib_lapor_anak(waktu_lapor);

-- Mempercepat dashboard analitik yang mengelompokkan data
CREATE INDEX IF NOT EXISTS idx_klien_kanwil_bapas_tipe ON klien(kanwil_id, bapas_id, tipe);
CREATE INDEX IF NOT EXISTS idx_riwayat_hukum_dewasa_kategori_tanggal ON riwayat_hukum_dewasa(kategori_tindak_pidana, tanggal_surat_keputusan_pengadilan);


-- =============================================================================
-- LANGKAH 5: OPTIMASI UNIQUE CONSTRAINT UNTUK SOFT DELETE
-- Menggantikan UNIQUE constraint biasa agar tidak konflik dengan data yang sudah di-soft-delete.
-- =============================================================================


-- =============================================================================