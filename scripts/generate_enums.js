const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

// === CONFIGURE THESE ===
const PG_CONFIG = {
    user: "aksara_admin",
    host: "127.0.0.1",
    database: "aksaradb",
    password: "password123",
    port: 5432,
};

const RUST_ENUMS_PATH = path.join(__dirname, '../backend', 'src', 'types.rs');
const TS_ENUMS_PATH = path.join(__dirname, '../frontend', 'src', 'lib', 'types.ts'); // optional

// === PG ENUM COLLECTOR QUERY ===
const ENUM_QUERY = `
SELECT t.typname as enum_name, e.enumlabel as enum_value
FROM pg_type t
JOIN pg_enum e ON t.oid = e.enumtypid
JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
ORDER BY enum_name, e.enumsortorder;
`;

async function main() {
    const client = new Client(PG_CONFIG);
    await client.connect();

    const res = await client.query(ENUM_QUERY);

    // Group by enum name
    const enums = {};
    for (let row of res.rows) {
        if (!enums[row.enum_name]) enums[row.enum_name] = [];
        enums[row.enum_name].push(row.enum_value);
    }

    // ==== Generate Rust ====
 let rustOut = "// AUTO-GENERATED FILE FROM DB ENUMS\n\n";
for (let [name, values] of Object.entries(enums)) {
    rustOut += renderRustEnum(name, values) + "\n";
}
    fs.mkdirSync(path.dirname(RUST_ENUMS_PATH), { recursive: true });
    fs.writeFileSync(RUST_ENUMS_PATH, rustOut.trimEnd());

    // ==== Generate TS (optional) ====
    let tsOut = "// AUTO-GENERATED FILE FROM DB ENUMS\n\n";
    for (let [name, values] of Object.entries(enums)) {
        tsOut += `export enum ${toPascalEnumName(name)} {\n`;
        for (let val of values) {
            tsOut += `    ${formatTsEnumKey(val)} = \"${val}\",\n`;
        }
        tsOut += "}\n\n";
    }
    fs.mkdirSync(path.dirname(TS_ENUMS_PATH), { recursive: true });
    fs.writeFileSync(TS_ENUMS_PATH, tsOut.trimEnd());

    await client.end();
    console.log('✅ Enums generated!');
}

function toPascal(str) {
    return str
        .split('_')
        .map(s => s.charAt(0).toUpperCase() + s.slice(1).toLowerCase())
        .join('');
}

function formatRustVariant(val) {
    // Replace non-word separators with spaces
    let cleaned = val.replace(/[-/]/g, ' ').replace(/[^a-zA-Z0-9 ]/g, '');
    // Split by spaces
    return cleaned.split(' ')
        .map(word =>
            word.length > 1 && word.toUpperCase() === word
                ? word // preserve acronyms like WNI, WNA
                : word.charAt(0).toUpperCase() + word.slice(1)
        )
        .join('');
}

// Helper for Rust enum type names (e.g. jenis_kelamin_enum -> JenisKelaminEnum)
function toPascalEnumName(str) {
    let result = str.split('_')
        .map(s => s.charAt(0).toUpperCase() + s.slice(1).toLowerCase())
        .join('');
    if (!result.endsWith('Enum')) result += 'Enum';
    return result;
}
function renderRustEnum(enumName, values) {
    let out = `#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize,Copy, Clone, PartialEq, Eq)]\n`;
    out += `#[sqlx(type_name = "${enumName}")]\n`;
    out += `pub enum ${toPascalEnumName(enumName)} {\n`;
    for (const val of values) {
        const variant = formatRustVariant(val);
        out += `    #[serde(rename = \"${val}\")]\n`;
        out += `    #[sqlx(rename = \"${val}\")]\n`;
        out += `    ${variant},\n`;
    }
    out += "}\n";
    return out;
}

// Helper for TS enum keys (e.g. 'belum/tidak bekerja' -> 'BelumTidakBekerja')
function formatTsEnumKey(val) {
    let cleaned = val.replace(/[-/]/g, ' ').replace(/[^a-zA-Z0-9 ]/g, '');
    return cleaned
        .split(' ')
        .map(word =>
            word.length > 1 && word.toUpperCase() === word
                ? word // preserve acronyms like WNI, WNA
                : word.charAt(0).toUpperCase() + word.slice(1)
        )
        .join('');
}


main().catch(e => {
    console.error(e);
    process.exit(1);
});