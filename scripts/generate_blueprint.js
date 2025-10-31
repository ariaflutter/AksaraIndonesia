const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

// === DB CONNECTION CONFIG ===
const PG_CONFIG = {
    user: "aksara_admin",
    host: "127.0.0.1",
    database: "aksaradb",
    password: "password123",
    port: 5432,
};

// === OUTPUT PATHS ===
const TS_BLUEPRINT_PATH = path.join(__dirname, 'blueprint.ts');
const RS_BLUEPRINT_PATH = path.join(__dirname, 'blueprint.rs');
const JSON_BLUEPRINT_PATH = path.join(__dirname, 'blueprint.json'); // Ganti dari .ts ke .json

// === SCHEMA QUERY ===
// Pulls columns for all public tables in deterministic order
const SCHEMA_QUERY = `
SELECT
  c.table_name,
  c.column_name,
  c.data_type,
  c.is_nullable,
  c.column_default,
  c.character_maximum_length,
  c.numeric_precision,
  c.numeric_scale,
  c.udt_name,
  t.table_type
FROM information_schema.columns c
JOIN information_schema.tables t
  ON c.table_schema = t.table_schema AND c.table_name = t.table_name
WHERE c.table_schema = 'public'
ORDER BY c.table_name, c.ordinal_position;
`;

async function main() {
    const client = new Client(PG_CONFIG);
    await client.connect();

    const res = await client.query(SCHEMA_QUERY);

    // Group by table name
    /** @type {Record<string, any[]>} */
    const tables = {};
    for (const row of res.rows) {
        const tableName = row.table_name;
        if (!tables[tableName]) tables[tableName] = [];
        tables[tableName].push({
            column: row.column_name,
            dataType: row.data_type,
            udtName: row.udt_name || null,
            isNullable: row.is_nullable === 'YES',
            columnDefault: row.column_default || null,
            characterMaximumLength: row.character_maximum_length == null ? null : Number(row.character_maximum_length),
            numericPrecision: row.numeric_precision == null ? null : Number(row.numeric_precision),
            numericScale: row.numeric_scale == null ? null : Number(row.numeric_scale),
            isEnum: (row.data_type === 'USER-DEFINED') || isLikelyEnum(row.udt_name),
            tableType: row.table_type,
        });
    }

    // === Write TypeScript blueprint ===
    const tsOut = renderTypescriptBlueprint(tables);
    fs.writeFileSync(TS_BLUEPRINT_PATH, tsOut);

    // === Write Rust blueprint ===
    const rsOut = renderRustBlueprint(tables);
    fs.writeFileSync(RS_BLUEPRINT_PATH, rsOut);

    const jsonOut = JSON.stringify(tables, null, 2);
    fs.writeFileSync(JSON_BLUEPRINT_PATH, jsonOut);

    await client.end();
    console.log('✅ Blueprint generated: scripts/blueprint.ts and scripts/blueprint.rs');
}

function isLikelyEnum(udtName) {
    if (!udtName) return false;
    // Heuristic: many projects suffix custom enum types with _enum
    return /_enum$/i.test(udtName);
}

function renderTypescriptBlueprint(tables) {
    let out = `// AUTO-GENERATED DB BLUEPRINT\n\n`;
    out += `export type TableColumn = {\n`;
    out += `  table: string;\n`;
    out += `  column: string;\n`;
    out += `  dataType: string;\n`;
    out += `  udtName?: string;\n`;
    out += `  isNullable: boolean;\n`;
    out += `  columnDefault?: string;\n`;
    out += `  characterMaximumLength?: number;\n`;
    out += `  numericPrecision?: number;\n`;
    out += `  numericScale?: number;\n`;
    out += `  isEnum: boolean;\n`;
    out += `  tableType: string;\n`;
    out += `};\n\n`;

    out += `export type DBSchema = Record<string, TableColumn[]>;\n\n`;
    out += `export const DB_SCHEMA: DBSchema = {\n`;

    const tableNames = Object.keys(tables);
    for (let i = 0; i < tableNames.length; i++) {
        const tbl = tableNames[i];
        out += `  ${escapeTsKey(tbl)}: [\n`;
        for (const col of tables[tbl]) {
            out += `    { table: "${tbl}", column: "${col.column}", dataType: "${col.dataType}", ` +
                   `udtName: ${renderOptStr(col.udtName)}, isNullable: ${col.isNullable}, columnDefault: ${renderOptStr(col.columnDefault)}, ` +
                   `characterMaximumLength: ${renderOptNum(col.characterMaximumLength)}, numericPrecision: ${renderOptNum(col.numericPrecision)}, ` +
                   `numericScale: ${renderOptNum(col.numericScale)}, isEnum: ${col.isEnum}, tableType: "${col.tableType}" },\n`;
        }
        out += `  ]${i === tableNames.length - 1 ? '' : ','}\n`;
    }
    out += `};\n`;
    return out;
}

function renderRustBlueprint(tables) {
    let out = `// AUTO-GENERATED DB BLUEPRINT\n\n`;
    out += `pub struct TableColumn {\n`;
    out += `    pub table: &'static str,\n`;
    out += `    pub column: &'static str,\n`;
    out += `    pub data_type: &'static str,\n`;
    out += `    pub udt_name: Option<&'static str>,\n`;
    out += `    pub is_nullable: bool,\n`;
    out += `    pub column_default: Option<&'static str>,\n`;
    out += `    pub character_maximum_length: Option<i32>,\n`;
    out += `    pub numeric_precision: Option<i32>,\n`;
    out += `    pub numeric_scale: Option<i32>,\n`;
    out += `    pub is_enum: bool,\n`;
    out += `    pub table_type: &'static str,\n`;
    out += `}\n\n`;

    out += `pub static DB_SCHEMA: &[TableColumn] = &[\n`;
    const tableNames = Object.keys(tables);
    for (const tbl of tableNames) {
        for (const col of tables[tbl]) {
            out += `    TableColumn { ` +
                   `table: "${tbl}", column: "${col.column}", data_type: "${col.dataType}", ` +
                   `udt_name: ${renderRsOptStr(col.udtName)}, is_nullable: ${col.isNullable}, column_default: ${renderRsOptStr(col.columnDefault)}, ` +
                   `character_maximum_length: ${renderRsOptNum(col.characterMaximumLength)}, numeric_precision: ${renderRsOptNum(col.numericPrecision)}, ` +
                   `numeric_scale: ${renderRsOptNum(col.numericScale)}, is_enum: ${col.isEnum}, table_type: "${col.tableType}" },\n`;
        }
    }
    out += `];\n`;
    return out;
}

function escapeTsKey(key) {
    // If key is not a valid TS identifier, quote it
    if (/^[A-Za-z_$][A-Za-z0-9_$]*$/.test(key)) return key;
    return JSON.stringify(key);
}

function renderOptStr(s) {
    return s == null ? 'undefined' : JSON.stringify(s);
}

function renderOptNum(n) {
    return n == null ? 'undefined' : String(n);
}

function renderRsOptStr(s) {
    return s == null ? 'None' : `Some("${escapeRsString(s)}")`;
}

function renderRsOptNum(n) {
    return n == null ? 'None' : `Some(${Number(n)})`;
}

function escapeRsString(s) {
    return String(s).replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});


