// AUTO-GENERATED FILE FROM DB ENUMS

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "jenis_kelamin_enum")]
pub enum JenisKelaminEnum {
    #[serde(rename = "Laki-laki")]
    #[sqlx(rename = "Laki-laki")]
    LakiLaki,
    #[serde(rename = "Perempuan")]
    #[sqlx(rename = "Perempuan")]
    Perempuan,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "jenis_pekerjaan_enum")]
pub enum JenisPekerjaanEnum {
    #[serde(rename = "Belum/Tidak Bekerja")]
    #[sqlx(rename = "Belum/Tidak Bekerja")]
    BelumTidakBekerja,
    #[serde(rename = "Pegawai Negeri Sipil")]
    #[sqlx(rename = "Pegawai Negeri Sipil")]
    PegawaiNegeriSipil,
    #[serde(rename = "Tentara Nasional Indonesia")]
    #[sqlx(rename = "Tentara Nasional Indonesia")]
    TentaraNasionalIndonesia,
    #[serde(rename = "Kepolisian RI")]
    #[sqlx(rename = "Kepolisian RI")]
    KepolisianRI,
    #[serde(rename = "Karyawan BUMN")]
    #[sqlx(rename = "Karyawan BUMN")]
    KaryawanBUMN,
    #[serde(rename = "Karyawan BUMD")]
    #[sqlx(rename = "Karyawan BUMD")]
    KaryawanBUMD,
    #[serde(rename = "Anggota DPR-RI")]
    #[sqlx(rename = "Anggota DPR-RI")]
    AnggotaDPRRI,
    #[serde(rename = "Anggota DPD")]
    #[sqlx(rename = "Anggota DPD")]
    AnggotaDPD,
    #[serde(rename = "Anggota BPK")]
    #[sqlx(rename = "Anggota BPK")]
    AnggotaBPK,
    #[serde(rename = "Presiden")]
    #[sqlx(rename = "Presiden")]
    Presiden,
    #[serde(rename = "Wakil Presiden")]
    #[sqlx(rename = "Wakil Presiden")]
    WakilPresiden,
    #[serde(rename = "Anggota Mahkamah Konstitusi")]
    #[sqlx(rename = "Anggota Mahkamah Konstitusi")]
    AnggotaMahkamahKonstitusi,
    #[serde(rename = "Anggota Kabinet/Kementerian")]
    #[sqlx(rename = "Anggota Kabinet/Kementerian")]
    AnggotaKabinetKementerian,
    #[serde(rename = "Duta Besar")]
    #[sqlx(rename = "Duta Besar")]
    DutaBesar,
    #[serde(rename = "Gubernur")]
    #[sqlx(rename = "Gubernur")]
    Gubernur,
    #[serde(rename = "Wakil Gubernur")]
    #[sqlx(rename = "Wakil Gubernur")]
    WakilGubernur,
    #[serde(rename = "Bupati")]
    #[sqlx(rename = "Bupati")]
    Bupati,
    #[serde(rename = "Wakil Bupati")]
    #[sqlx(rename = "Wakil Bupati")]
    WakilBupati,
    #[serde(rename = "Walikota")]
    #[sqlx(rename = "Walikota")]
    Walikota,
    #[serde(rename = "Wakil Walikota")]
    #[sqlx(rename = "Wakil Walikota")]
    WakilWalikota,
    #[serde(rename = "Anggota DPRD Provinsi")]
    #[sqlx(rename = "Anggota DPRD Provinsi")]
    AnggotaDPRDProvinsi,
    #[serde(rename = "Anggota DPRD Kabupaten/Kota")]
    #[sqlx(rename = "Anggota DPRD Kabupaten/Kota")]
    AnggotaDPRDKabupatenKota,
    #[serde(rename = "Pengacara")]
    #[sqlx(rename = "Pengacara")]
    Pengacara,
    #[serde(rename = "Notaris")]
    #[sqlx(rename = "Notaris")]
    Notaris,
    #[serde(rename = "Peneliti")]
    #[sqlx(rename = "Peneliti")]
    Peneliti,
    #[serde(rename = "Perangkat Desa")]
    #[sqlx(rename = "Perangkat Desa")]
    PerangkatDesa,
    #[serde(rename = "Kepala Desa")]
    #[sqlx(rename = "Kepala Desa")]
    KepalaDesa,
    #[serde(rename = "Dosen")]
    #[sqlx(rename = "Dosen")]
    Dosen,
    #[serde(rename = "Guru")]
    #[sqlx(rename = "Guru")]
    Guru,
    #[serde(rename = "Perdagangan")]
    #[sqlx(rename = "Perdagangan")]
    Perdagangan,
    #[serde(rename = "Industri")]
    #[sqlx(rename = "Industri")]
    Industri,
    #[serde(rename = "Konstruksi")]
    #[sqlx(rename = "Konstruksi")]
    Konstruksi,
    #[serde(rename = "Transportasi")]
    #[sqlx(rename = "Transportasi")]
    Transportasi,
    #[serde(rename = "Karyawan Swasta")]
    #[sqlx(rename = "Karyawan Swasta")]
    KaryawanSwasta,
    #[serde(rename = "Karyawan Honorer")]
    #[sqlx(rename = "Karyawan Honorer")]
    KaryawanHonorer,
    #[serde(rename = "Buruh Harian Lepas")]
    #[sqlx(rename = "Buruh Harian Lepas")]
    BuruhHarianLepas,
    #[serde(rename = "Pembantu Rumah Tangga")]
    #[sqlx(rename = "Pembantu Rumah Tangga")]
    PembantuRumahTangga,
    #[serde(rename = "Tukang Cukur")]
    #[sqlx(rename = "Tukang Cukur")]
    TukangCukur,
    #[serde(rename = "Tukang Listrik")]
    #[sqlx(rename = "Tukang Listrik")]
    TukangListrik,
    #[serde(rename = "Tukang Batu")]
    #[sqlx(rename = "Tukang Batu")]
    TukangBatu,
    #[serde(rename = "Tukang Kayu")]
    #[sqlx(rename = "Tukang Kayu")]
    TukangKayu,
    #[serde(rename = "Tukang Sol Sepatu")]
    #[sqlx(rename = "Tukang Sol Sepatu")]
    TukangSolSepatu,
    #[serde(rename = "Tukang Las/Pandai Besi")]
    #[sqlx(rename = "Tukang Las/Pandai Besi")]
    TukangLasPandaiBesi,
    #[serde(rename = "Tukang Jahit")]
    #[sqlx(rename = "Tukang Jahit")]
    TukangJahit,
    #[serde(rename = "Tukang Gigi")]
    #[sqlx(rename = "Tukang Gigi")]
    TukangGigi,
    #[serde(rename = "Penata Rias")]
    #[sqlx(rename = "Penata Rias")]
    PenataRias,
    #[serde(rename = "Penata Busana")]
    #[sqlx(rename = "Penata Busana")]
    PenataBusana,
    #[serde(rename = "Penata Rambut")]
    #[sqlx(rename = "Penata Rambut")]
    PenataRambut,
    #[serde(rename = "Mekanik")]
    #[sqlx(rename = "Mekanik")]
    Mekanik,
    #[serde(rename = "Seniman")]
    #[sqlx(rename = "Seniman")]
    Seniman,
    #[serde(rename = "Tabib")]
    #[sqlx(rename = "Tabib")]
    Tabib,
    #[serde(rename = "Paraji")]
    #[sqlx(rename = "Paraji")]
    Paraji,
    #[serde(rename = "Perancang Busana")]
    #[sqlx(rename = "Perancang Busana")]
    PerancangBusana,
    #[serde(rename = "Penterjemah")]
    #[sqlx(rename = "Penterjemah")]
    Penterjemah,
    #[serde(rename = "Wartawan")]
    #[sqlx(rename = "Wartawan")]
    Wartawan,
    #[serde(rename = "Juru Masak")]
    #[sqlx(rename = "Juru Masak")]
    JuruMasak,
    #[serde(rename = "Promotor Acara")]
    #[sqlx(rename = "Promotor Acara")]
    PromotorAcara,
    #[serde(rename = "Pilot")]
    #[sqlx(rename = "Pilot")]
    Pilot,
    #[serde(rename = "Arsitek")]
    #[sqlx(rename = "Arsitek")]
    Arsitek,
    #[serde(rename = "Akuntan")]
    #[sqlx(rename = "Akuntan")]
    Akuntan,
    #[serde(rename = "Konsultan")]
    #[sqlx(rename = "Konsultan")]
    Konsultan,
    #[serde(rename = "Penyiar Televisi")]
    #[sqlx(rename = "Penyiar Televisi")]
    PenyiarTelevisi,
    #[serde(rename = "Penyiar Radio")]
    #[sqlx(rename = "Penyiar Radio")]
    PenyiarRadio,
    #[serde(rename = "Pelaut")]
    #[sqlx(rename = "Pelaut")]
    Pelaut,
    #[serde(rename = "Sopir")]
    #[sqlx(rename = "Sopir")]
    Sopir,
    #[serde(rename = "Pialang")]
    #[sqlx(rename = "Pialang")]
    Pialang,
    #[serde(rename = "Paranormal")]
    #[sqlx(rename = "Paranormal")]
    Paranormal,
    #[serde(rename = "Pedagang")]
    #[sqlx(rename = "Pedagang")]
    Pedagang,
    #[serde(rename = "Wiraswasta")]
    #[sqlx(rename = "Wiraswasta")]
    Wiraswasta,
    #[serde(rename = "Petani/Pekebun")]
    #[sqlx(rename = "Petani/Pekebun")]
    PetaniPekebun,
    #[serde(rename = "Peternak")]
    #[sqlx(rename = "Peternak")]
    Peternak,
    #[serde(rename = "Buruh Tani/Perkebunan")]
    #[sqlx(rename = "Buruh Tani/Perkebunan")]
    BuruhTaniPerkebunan,
    #[serde(rename = "Buruh Peternakan")]
    #[sqlx(rename = "Buruh Peternakan")]
    BuruhPeternakan,
    #[serde(rename = "Nelayan/Perikanan")]
    #[sqlx(rename = "Nelayan/Perikanan")]
    NelayanPerikanan,
    #[serde(rename = "Buruh Nelayan/Perikanan")]
    #[sqlx(rename = "Buruh Nelayan/Perikanan")]
    BuruhNelayanPerikanan,
    #[serde(rename = "Imam Mesjid")]
    #[sqlx(rename = "Imam Mesjid")]
    ImamMesjid,
    #[serde(rename = "Pendeta")]
    #[sqlx(rename = "Pendeta")]
    Pendeta,
    #[serde(rename = "Pastor")]
    #[sqlx(rename = "Pastor")]
    Pastor,
    #[serde(rename = "Ustadz/Mubaligh")]
    #[sqlx(rename = "Ustadz/Mubaligh")]
    UstadzMubaligh,
    #[serde(rename = "Biarawati")]
    #[sqlx(rename = "Biarawati")]
    Biarawati,
    #[serde(rename = "Pelajar/Mahasiswa")]
    #[sqlx(rename = "Pelajar/Mahasiswa")]
    PelajarMahasiswa,
    #[serde(rename = "Dokter")]
    #[sqlx(rename = "Dokter")]
    Dokter,
    #[serde(rename = "Bidan")]
    #[sqlx(rename = "Bidan")]
    Bidan,
    #[serde(rename = "Perawat")]
    #[sqlx(rename = "Perawat")]
    Perawat,
    #[serde(rename = "Apoteker")]
    #[sqlx(rename = "Apoteker")]
    Apoteker,
    #[serde(rename = "Psikiater/Psikolog")]
    #[sqlx(rename = "Psikiater/Psikolog")]
    PsikiaterPsikolog,
    #[serde(rename = "Pensiunan")]
    #[sqlx(rename = "Pensiunan")]
    Pensiunan,
    #[serde(rename = "Mengurus Rumah Tangga")]
    #[sqlx(rename = "Mengurus Rumah Tangga")]
    MengurusRumahTangga,
    #[serde(rename = "Lainnya")]
    #[sqlx(rename = "Lainnya")]
    Lainnya,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "kewarganegaraan_enum")]
pub enum KewarganegaraanEnum {
    #[serde(rename = "WNI")]
    #[sqlx(rename = "WNI")]
    WNI,
    #[serde(rename = "WNA")]
    #[sqlx(rename = "WNA")]
    WNA,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "metode_lapor_enum")]
pub enum MetodeLaporEnum {
    #[serde(rename = "Online")]
    #[sqlx(rename = "Online")]
    Online,
    #[serde(rename = "Self-Service")]
    #[sqlx(rename = "Self-Service")]
    SelfService,
    #[serde(rename = "Petugas")]
    #[sqlx(rename = "Petugas")]
    Petugas,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "nama_instansi_enum")]
pub enum NamaInstansiEnum {
    #[serde(rename = "Lembaga Pemasyarakatan")]
    #[sqlx(rename = "Lembaga Pemasyarakatan")]
    LembagaPemasyarakatan,
    #[serde(rename = "Rumah Tahanan Negara")]
    #[sqlx(rename = "Rumah Tahanan Negara")]
    RumahTahananNegara,
    #[serde(rename = "Balai Pemasyarakatan")]
    #[sqlx(rename = "Balai Pemasyarakatan")]
    BalaiPemasyarakatan,
    #[serde(rename = "Kejaksaan Negeri")]
    #[sqlx(rename = "Kejaksaan Negeri")]
    KejaksaanNegeri,
    #[serde(rename = "Pengadilan Negeri")]
    #[sqlx(rename = "Pengadilan Negeri")]
    PengadilanNegeri,
    #[serde(rename = "Kepolisian Resor")]
    #[sqlx(rename = "Kepolisian Resor")]
    KepolisianResor,
    #[serde(rename = "Kepolisian Sektor")]
    #[sqlx(rename = "Kepolisian Sektor")]
    KepolisianSektor,
    #[serde(rename = "Kepolisian Daerah")]
    #[sqlx(rename = "Kepolisian Daerah")]
    KepolisianDaerah,
    #[serde(rename = "Kepolisian Republik Indonesia")]
    #[sqlx(rename = "Kepolisian Republik Indonesia")]
    KepolisianRepublikIndonesia,
    #[serde(rename = "Pengadilan Tinggi")]
    #[sqlx(rename = "Pengadilan Tinggi")]
    PengadilanTinggi,
    #[serde(rename = "Mahkamah Agung")]
    #[sqlx(rename = "Mahkamah Agung")]
    MahkamahAgung,
    #[serde(rename = "Lainnya")]
    #[sqlx(rename = "Lainnya")]
    Lainnya,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "tingkat_pendidikan_enum")]
pub enum TingkatPendidikanEnum {
    #[serde(rename = "Tidak Sekolah")]
    #[sqlx(rename = "Tidak Sekolah")]
    TidakSekolah,
    #[serde(rename = "SD Tidak Lulus")]
    #[sqlx(rename = "SD Tidak Lulus")]
    SDTidakLulus,
    #[serde(rename = "SD atau Sederajat")]
    #[sqlx(rename = "SD atau Sederajat")]
    SDAtauSederajat,
    #[serde(rename = "SMP atau Sederajat")]
    #[sqlx(rename = "SMP atau Sederajat")]
    SMPAtauSederajat,
    #[serde(rename = "SMA atau Sederajat")]
    #[sqlx(rename = "SMA atau Sederajat")]
    SMAAtauSederajat,
    #[serde(rename = "D1 atau Sederajat")]
    #[sqlx(rename = "D1 atau Sederajat")]
    D1AtauSederajat,
    #[serde(rename = "D2 atau Sederajat")]
    #[sqlx(rename = "D2 atau Sederajat")]
    D2AtauSederajat,
    #[serde(rename = "D3 atau Sederajat")]
    #[sqlx(rename = "D3 atau Sederajat")]
    D3AtauSederajat,
    #[serde(rename = "S1 atau Sederajat")]
    #[sqlx(rename = "S1 atau Sederajat")]
    S1AtauSederajat,
    #[serde(rename = "S2 atau Sederajat")]
    #[sqlx(rename = "S2 atau Sederajat")]
    S2AtauSederajat,
    #[serde(rename = "S3 atau Sederajat")]
    #[sqlx(rename = "S3 atau Sederajat")]
    S3AtauSederajat,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "tipe_klien_enum")]
pub enum TipeKlienEnum {
    #[serde(rename = "Dewasa")]
    #[sqlx(rename = "Dewasa")]
    Dewasa,
    #[serde(rename = "Anak")]
    #[sqlx(rename = "Anak")]
    Anak,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "user_role_enum")]
pub enum UserRoleEnum {
    #[serde(rename = "Pegawai")]
    #[sqlx(rename = "Pegawai")]
    Pegawai,
    #[serde(rename = "AdminBapas")]
    #[sqlx(rename = "AdminBapas")]
    AdminBapas,
    #[serde(rename = "AdminKanwil")]
    #[sqlx(rename = "AdminKanwil")]
    AdminKanwil,
    #[serde(rename = "SuperAdmin")]
    #[sqlx(rename = "SuperAdmin")]
    SuperAdmin,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "user_status_aktif_enum")]
pub enum UserStatusAktifEnum {
    #[serde(rename = "Aktif")]
    #[sqlx(rename = "Aktif")]
    Aktif,
    #[serde(rename = "Deaktif")]
    #[sqlx(rename = "Deaktif")]
    Deaktif,
}

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "user_status_kepegawaian_enum")]
pub enum UserStatusKepegawaianEnum {
    #[serde(rename = "Aktif")]
    #[sqlx(rename = "Aktif")]
    Aktif,
    #[serde(rename = "Pindah Jabatan")]
    #[sqlx(rename = "Pindah Jabatan")]
    PindahJabatan,
    #[serde(rename = "Pensiun")]
    #[sqlx(rename = "Pensiun")]
    Pensiun,
    #[serde(rename = "Lainnya")]
    #[sqlx(rename = "Lainnya")]
    Lainnya,
}