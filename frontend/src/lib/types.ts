// src/lib/types.ts

// NOTE: The Rust enums are represented here as string literal types.
// You MUST ensure these string values match exactly what your Rust
// enums serialize to. For example, if your Rust `UserRole` enum has
// variants `Admin` and `Pk`, the strings here should be 'Admin' and 'Pk'.

// From src/types/*
export type UserRole = 'Admin' | 'Pk' | 'SuperAdmin'; // Example values
export type TipeKlien = 'Dewasa' | 'Anak'; // Example values
export type JenisPekerjaan = 'PNS' | 'Swasta' | 'Wiraswasta' | 'Lainnya'; // Example values
export type TingkatPendidikan = 'SD' | 'SMP' | 'SMA' | 'S1' | 'S2' | 'S3'; // Example values
export type NamaInstansi = 'Polri' | 'Kejaksaan' | 'Pengadilan' | 'Lapas'; // Example values
export type MetodeLapor = 'Petugas' | 'Online'; // Example values

// --- Auth Types (from src/auth/model.rs) ---

export interface LoginRequest {
    nip: string;
    password: string;
}

export interface LoginResponse {
    token: string;
}

export interface Claims {
    sub: number; // User ID
    role: UserRole;
    unit_kerja_id: number | null;
    exp: number; // Expiration timestamp
}

// --- Bapas Types (from src/bapas/model.rs) ---

export interface Bapas {
    id: number;
    nama_bapas: string;
    kota: string;
    alamat: string | null;
    nomor_telepon_bapas: string | null;
    email: string | null;
    kanwil: string | null;
}

// Payload for creating a new Bapas
export interface CreateBapasPayload {
    nama_bapas: string;
    kota: string;
    alamat?: string | null;
    nomor_telepon_bapas?: string | null;
    email?: string | null;
    kanwil?: string | null;
}

// --- Klien Core Types (from src/klien/model_core.rs) ---

export interface Klien {
    id: number;
    tipe: TipeKlien;
    nama: string;
    alamat: string | null;
    tempat_lahir: string | null;
    tanggal_lahir: string | null; // ISO 8601 date string (e.g., "YYYY-MM-DD")
    jenis_kelamin: string | null;
    agama: string | null;
    pekerjaan: JenisPekerjaan | null;
    pendidikan_terakhir: TingkatPendidikan | null;
    bapas_id: number;
    pk_id: number;
    online_akses: boolean;
    created_at: string; // ISO 8601 datetime string
    updated_at: string; // ISO 8601 datetime string
    created_by: number | null;
    updated_by: number | null;
}

export interface CreateKlienPayload {
    tipe: TipeKlien;
    nama: string;
    alamat?: string | null;
    tempat_lahir?: string | null;
    tanggal_lahir?: string | null;
    jenis_kelamin?: string | null;
    agama?: string | null;
    pekerjaan?: JenisPekerjaan | null;
    pendidikan_terakhir?: TingkatPendidikan | null;
    bapas_id?: number | null;
    pk_id?: number | null;
}

export interface UpdateKlienPayload {
    tipe?: TipeKlien;
    nama?: string;
    alamat?: string;
    tempat_lahir?: string;
    tanggal_lahir?: string;
    jenis_kelamin?: string;
    agama?: string;
    pekerjaan?: JenisPekerjaan;
    pendidikan_terakhir?: TingkatPendidikan;
    bapas_id?: number;
    pk_id?: number;
    online_akses?: boolean;
}


// --- Klien Dewasa Sub-Types ---

export interface PenerimaanDewasa {
    id: number;
    klien_id: number;
    tanggal_permintaan_lapas: string | null;
    tanggal_surat_tugas: string | null;
    perihal: string | null;
    no_register_litmas: string | null;
    nomor_surat_permintaan_lapas: string | null;
    jenis_permintaan_litmas_lapas: string | null;
    nama_instansi: NamaInstansi | null;
    kelas_instansi: string | null;
    daerah_instansi: string | null;
    nama_penjamin: string | null;
    alamat_penjamin: string | null;
    created_at: string;
    updated_at: string;
    created_by: number | null;
    updated_by: number | null;
}

export interface CreatePenerimaanDewasaPayload {
    tanggal_permintaan_lapas?: string | null;
    tanggal_surat_tugas?: string | null;
    perihal?: string | null;
    no_register_litmas?: string | null;
    nomor_surat_permintaan_lapas?: string | null;
    jenis_permintaan_litmas_lapas?: string | null;
    nama_instansi?: NamaInstansi | null;
    kelas_instansi?: string | null;
    daerah_instansi?: string | null;
    nama_penjamin?: string | null;
    alamat_penjamin?: string | null;
}

export interface UpdatePenerimaanDewasaPayload extends CreatePenerimaanDewasaPayload {}

export interface RiwayatHukumDewasa {
    id: number;
    klien_id: number;
    kategori_tindak_pidana: string | null;
    pasal_tindak_pidana: string | null;
    tanggal_surat_keputusan_pengadilan: string | null;
    nomor_surat_keputusan_pengadilan: string | null;
    pidana_tahun: number | null;
    pidana_bulan: number | null;
    pidana_hari: number | null;
    pertama_ditahan: string | null;
    created_at: string;
    updated_at: string;
    created_by: number | null;
    updated_by: number | null;
}

export interface CreateRiwayatHukumDewasaPayload {
    kategori_tindak_pidana?: string | null;
    pasal_tindak_pidana?: string | null;
    tanggal_surat_keputusan_pengadilan?: string | null;
    nomor_surat_keputusan_pengadilan?: string | null;
    pidana_tahun?: number | null;
    pidana_bulan?: number | null;
    pidana_hari?: number | null;
    pertama_ditahan?: string | null;
}

export interface UpdateRiwayatHukumDewasaPayload extends CreateRiwayatHukumDewasaPayload {}


export interface LayananIntegrasiDewasa {
    id: number;
    klien_id: number;
    nomor_sk: string | null;
    tanggal_sk: string | null;
    nomor_register_integrasi: string | null;
    masa_bimbingan_awal: string | null;
    masa_bimbingan_akhir: string | null;
    petugas_layanan_id: number | null;
    created_at: string;
    updated_at: string;
    created_by: number | null;
    updated_by: number | null;
}

export interface CreateLayananIntegrasiDewasaPayload {
    nomor_sk?: string | null;
    tanggal_sk?: string | null;
    nomor_register_integrasi?: string | null;
    masa_bimbingan_awal?: string | null;
    masa_bimbingan_akhir?: string | null;
    petugas_layanan_id?: number | null;
}

export interface UpdateLayananIntegrasiDewasaPayload extends CreateLayananIntegrasiDewasaPayload {}


export interface WajibLaporDewasa {
    id: number; // i64 becomes number
    klien_id: number;
    waktu_lapor: string; // DateTime becomes string
    photo_path: string | null;
    latitude: string | null; // Decimal becomes string for precision
    longitude: string | null; // Decimal becomes string for precision
    metode_lapor: MetodeLapor;
    created_at: string;
}

export interface CreateWajibLaporDewasaPayload {
    photo_path?: string | null;
    latitude?: string | null;
    longitude?: string | null;
    // metode_lapor is handled by the backend default
}