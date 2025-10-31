// File: scripts/generate_models.js

const fs = require('fs');
const path = require('path');
// Impor blueprint yang sudah dibuat. Ini adalah satu-satunya dependensi eksternal.
const DB_SCHEMA = require('./blueprint.json');

const BACKEND_SRC_PATH = path.join(__dirname, '../backend', 'src');

console.log("Running generate_models.js...");
// =============================================================================
// HELPER FUNCTIONS (Duplicated from generate_enums.js for isolation)
// =============================================================================

function toPascalCase(str) {
    return str.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');
}

function toPascalEnumName(str) {
    let result = str.split('_')
        .map(s => s.charAt(0).toUpperCase() + s.slice(1).toLowerCase())
        .join('');
    if (!result.endsWith('Enum')) result += 'Enum';
    return result;
}

// =============================================================================
// TYPE MAPPING LOGIC
// =============================================================================

function pgTypeToRust(col, options = {}) {
    const { isUpdate = false, isCreate = false } = options;
    let rustType;

    switch (col.dataType) {
        case 'integer': rustType = 'i32'; break;
        case 'bigint': rustType = 'i64'; break;
        case 'text':
        case 'character varying': rustType = 'String'; break;
        case 'boolean': rustType = 'bool'; break;
        case 'timestamp with time zone': rustType = 'chrono::DateTime<chrono::Utc>'; break;
        case 'date': rustType = 'chrono::NaiveDate'; break;
        case 'numeric': rustType = 'rust_decimal::Decimal'; break;
        case 'USER-DEFINED':
            rustType = toPascalEnumName(col.udtName);
            break;
        default: rustType = 'String'; // Default fallback aman
    }
    
    // Untuk struct UPDATE, semua field harus Option<T>
    if (isUpdate) {
        return `Option<${rustType}>`;
    }

    let isNullable = col.isNullable;
    // Untuk struct CREATE, field bisa opsional jika DB punya nilai DEFAULT
    if (isCreate && col.columnDefault) {
        isNullable = true;
    }

    return isNullable ? `Option<${rustType}>` : rustType;
}


// =============================================================================
// STRUCT GENERATION LOGIC
// =============================================================================

function generateStructsForTable(tableName, columns) {
    const structName = toPascalCase(tableName);
    let output = `// === ${structName} Models ===\n\n`;

    // --- Generate Struct Utama (Read) ---
    output += `#[derive(Debug, serde::Serialize, sqlx::FromRow)]\n`;
    output += `pub struct ${structName} {\n`;
    for (const col of columns) {
        output += `    pub ${col.column}: ${pgTypeToRust(col)},\n`;
    }
    output += `}\n\n`;

    // --- Generate Struct Create (Write) ---
    // Daftar kolom yang diisi otomatis oleh DB dan harus diabaikan saat CREATE
    const autoGenColsCreate = ['id', 'created_at', 'updated_at', 'deleted_at', 'created_by', 'updated_by'];
    // Untuk tabel 'klien', 'bapas_id' dan 'kanwil_id' diisi oleh trigger, jadi abaikan.
    if (tableName === 'klien') autoGenColsCreate.push('bapas_id', 'kanwil_id');
    // Untuk 'proses_hukum', 'klien_id' diisi oleh trigger.
    if (tableName.startsWith('proses_hukum')) autoGenColsCreate.push('klien_id');

    output += `#[derive(Debug, serde::Deserialize)]\n`;
    output += `pub struct Create${structName} {\n`;
    for (const col of columns) {
        if (autoGenColsCreate.includes(col.column)) continue;
        output += `    pub ${col.column}: ${pgTypeToRust(col, { isCreate: true })},\n`;
    }
    output += `}\n\n`;
    
    // --- Generate Struct Update (Write) ---
    const autoGenColsUpdate = ['id', 'created_at', 'deleted_at', 'created_by'];
    if (tableName === 'klien') autoGenColsUpdate.push('bapas_id', 'kanwil_id');
    if (tableName.startsWith('proses_hukum')) autoGenColsUpdate.push('klien_id');

    output += `#[derive(Debug, serde::Deserialize)]\n`;
    output += `pub struct Update${structName} {\n`;
    for (const col of columns) {
        // 'updated_by' diisi oleh handler, bukan payload
        if (autoGenColsUpdate.includes(col.column) || col.column === 'updated_by') continue;
        output += `    pub ${col.column}: ${pgTypeToRust(col, { isUpdate: true })},\n`;
    }
    output += `}\n\n`;
    return output;
    
}


// =============================================================================
// MAIN EXECUTION LOGIC
// =============================================================================

function main() {
    // Petakan tabel ke modulnya (misal: tabel 'klien' ada di modul 'klien')
    const tableToModuleMap = {
        kanwil: 'kanwil',
        bapas: 'bapas',
        users: 'users',
        klien: 'klien',
        penerimaan_dewasa: 'klien',
        riwayat_hukum_dewasa: 'klien',
        layanan_integrasi_dewasa: 'klien',
        proses_hukum_dewasa: 'klien',
        wajib_lapor_dewasa: 'klien',
        // Tambahkan tabel anak dan lainnya di sini
    };
    
    const moduleFiles = {};

    for (const tableName in DB_SCHEMA) {
        const moduleName = tableToModuleMap[tableName];
        if (!moduleName) continue; // Abaikan tabel internal atau yang tidak dipetakan

        // Tentukan nama file tujuan
        let targetFile;
        if (tableName.includes('_dewasa')) targetFile = 'model_dewasa.rs';
        else if (tableName.includes('_anak')) targetFile = 'model_anak.rs';
        else targetFile = 'model_core.rs';
        
        const filePath = path.join(moduleName, targetFile);
        
        if (!moduleFiles[filePath]) {
            moduleFiles[filePath] = '';
        }

        moduleFiles[filePath] += generateStructsForTable(tableName, DB_SCHEMA[tableName]);
    }

    // Tulis semua konten yang sudah dikelompokkan ke file yang benar
    for (const filePath in moduleFiles) {
        const fullPath = path.join(BACKEND_SRC_PATH, filePath);
        // Tambahkan import yang umum di bagian atas setiap file
        let fileContent = `// AUTO-GENERATED MODELS FROM DB SCHEMA\n\n`;
        fileContent += `use serde::{Deserialize, Serialize};\n`;
        fileContent += `use sqlx::FromRow;\n`;
        fileContent += `use chrono::{DateTime, Utc, NaiveDate};\n`;
        fileContent += `use rust_decimal::Decimal;\n`;
        fileContent += `use crate::types::*;\n\n`; // Impor semua enum
        fileContent += moduleFiles[filePath];

        fs.mkdirSync(path.dirname(fullPath), { recursive: true });
        fs.writeFileSync(fullPath, fileContent);
        console.log(`✅ Wrote ${fullPath}`);
    }
}


// Jalankan skrip
if (require.main === module) {
    // Hanya jalankan langsung jika file ini dipanggil lewat CLI
    main();
}

// Ekspor fungsi supaya bisa dipanggil dari file lain
module.exports = {
    main,
    generateStructsForTable,
    pgTypeToRust,
    toPascalCase,
    toPascalEnumName
};