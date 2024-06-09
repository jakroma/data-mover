pub const NOT_NULL: &str = "NOT NULL";
pub const SERIAL_PRIMARY_KEY: &str = "SERIAL PRIMARY KEY";
pub const DEFAULT_SCHEMA: &str = "public";
pub const POSTGRESQL_SCHEMA_TABLE: &str = "SELECT table_schema, table_name FROM information_schema.tables";
pub const POSTGRESQL_SCHEMA_TABLE_WHERE: &str = "SELECT table_schema, table_name FROM information_schema.tables WHERE table_schema = $1";
pub const POSTGRESQL_COLUMNS_QUERY: &str = "
WITH fk_info AS (
    SELECT
        fk.table_schema AS table_schema,
        fk.table_name AS table_name,
        fk.column_name AS column_name,
        pk.table_schema AS foreign_table_schema,
        pk.table_name AS foreign_table_name,
        pk.column_name AS foreign_column_name
    FROM
        information_schema.key_column_usage AS fk
    JOIN
        information_schema.referential_constraints AS rc
        ON fk.constraint_name = rc.constraint_name
    JOIN
        information_schema.key_column_usage AS pk
        ON rc.unique_constraint_name = pk.constraint_name
        AND fk.ordinal_position = pk.ordinal_position
)
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
        WHEN fk_info.foreign_table_name IS NOT NULL THEN true
        ELSE false
    END as is_foreign_key,
    CASE
        WHEN fk_info.foreign_table_name IS NOT NULL THEN fk_info.foreign_table_schema || '.' || fk_info.foreign_table_name
        ELSE NULL
    END as foreign_table_full_name,
    fk_info.foreign_column_name
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
    fk_info ON c.table_schema = fk_info.table_schema 
    AND c.table_name = fk_info.table_name 
    AND c.column_name = fk_info.column_name
WHERE 
    c.table_schema = $1 
    AND c.table_name = $2
ORDER BY 
    c.ordinal_position;
";
