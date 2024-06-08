pub const NOT_NULL: &str = "NOT NULL";
pub const SERIAL_PRIMARY_KEY: &str = "SERIAL PRIMARY KEY";
pub const DEFAULT_SCHEMA: &str = "public";
pub const POSTGRESQL_SCHEMA_TABLE: &str = "SELECT table_schema, table_name FROM information_schema.tables";
pub const POSTGRESQL_SCHEMA_TABLE_WHERE: &str = "SELECT table_schema, table_name FROM information_schema.tables WHERE table_schema = $1";
pub const POSTGRESQL_COLUMNS_QUERY: &str = "
SELECT 
    c.table_schema,
    c.table_name,
    c.column_name,
    c.data_type,
    CASE
        WHEN c.is_nullable = 'YES' THEN true
        ELSE false
    END as is_nullable,
    CASE
        WHEN tc.constraint_type = 'PRIMARY KEY' THEN true
        ELSE false
    END as is_primary_key,
    CASE
        WHEN fkc.constraint_type = 'FOREIGN KEY' THEN true
        ELSE false
    END as is_foreign_key,
    fkcu.table_name AS foreign_table_name,
    fkcu.column_name AS foreign_column_name
FROM 
    information_schema.columns c
LEFT JOIN 
    information_schema.key_column_usage kcu 
    ON c.table_schema = kcu.table_schema 
    AND c.table_name = kcu.table_name 
    AND c.column_name = kcu.column_name
LEFT JOIN 
    information_schema.table_constraints tc 
    ON kcu.constraint_name = tc.constraint_name 
    AND kcu.table_schema = tc.table_schema 
    AND kcu.table_name = tc.table_name 
    AND tc.constraint_type = 'PRIMARY KEY'
LEFT JOIN 
    information_schema.table_constraints fkc 
    ON kcu.constraint_name = fkc.constraint_name 
    AND kcu.table_schema = fkc.table_schema 
    AND kcu.table_name = fkc.table_name 
    AND fkc.constraint_type = 'FOREIGN KEY'
LEFT JOIN 
    information_schema.key_column_usage fkcu 
    ON fkc.constraint_name = fkcu.constraint_name 
    AND fkc.table_schema = fkcu.table_schema 
    AND fkcu.position_in_unique_constraint IS NOT NULL
WHERE 
c.table_schema = $1 AND c.table_name = $2
ORDER BY 
    c.ordinal_position;
";
