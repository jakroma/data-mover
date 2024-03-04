use std::{
    fs::{File},
    io::Write,
    path::Path,
};

use crate::{db::postgresql::postgresql::TableInfo, DMResult};

pub fn write_definition(
    table_info: &TableInfo,
    file_path: &Path,
) -> DMResult<()> {
    let mut file = File::create(file_path)?;
    let mut id = 0;

    write!(file, "{{")?;
    write!(file, "\"schema\":{:?},", table_info.table_schema)?;
    write!(file, "\"containerName\":{:?},", table_info.table_name)?;
    write!(file, "\"columns\":[")?;
    for column in &table_info.columns {
        id = id + 1;
        write!(file, "{{")?;
        write!(file, "\"columnName\":{:?},", column.column_name)?;
        write!(file, "\"dataType\":{:?},", column.data_type)?;
        write!(file, "\"isNullable\":{:?},", column.is_nullable)?;
        write!(file, "\"isPrimaryKey\":{:?}", column.is_primary_key)?;
        write!(file, "\"foreignTableName\":{:?},", column.foreign_table_name)?;
        write!(file, "\"foreignColumnName\":{:?}", column.foreign_column_name)?;
        write!(file, "}},")?;
    }
    write!(file, "]")?;
    write!(file, "}}")?;

    Ok(()) 
}