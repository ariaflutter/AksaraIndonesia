-- Migration: Implement Soft Deletes

-- Menambahkan kolom deleted_at ke tabel-tabel inti
ALTER TABLE kanwil ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE bapas ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE klien ADD COLUMN deleted_at TIMESTAMPTZ;

-- Menambahkan ke tabel workflow Dewasa
ALTER TABLE penerimaan_dewasa ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE riwayat_hukum_dewasa ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE layanan_integrasi_dewasa ADD COLUMN deleted_at TIMESTAMPTZ;

-- Menambahkan ke tabel workflow Anak
ALTER TABLE penerimaan_anak ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE riwayat_hukum_anak ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE layanan_integrasi_anak ADD COLUMN deleted_at TIMESTAMPTZ;