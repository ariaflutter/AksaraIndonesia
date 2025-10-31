-- Add migration script here
ALTER TABLE wajib_lapor_dewasa
ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Untuk wajib_lapor_anak
ALTER TABLE wajib_lapor_anak
ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
