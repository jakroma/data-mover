use std::{
    fs::{File},
    io::Write,
    path::Path,
};

use crate::{models::data_definitions::DataDefinition, DMResult};

pub fn write_definition(
    table_info: &DataDefinition,
    file_path: &Path,
) -> DMResult<()> {
    let mut file = File::create(file_path)?;
    let mut id = 0;

    write!(file, "{{")?;
    write!(file, "\"containerName\":{:?},", table_info.data_container_name)?;
    write!(file, "\"columns\":[")?;
    for column in &table_info.property_info {
        id = id + 1;
        write!(file, "{{")?;
        write!(file, "\"columnName\":{:?},", column.property_name)?;
        write!(file, "\"dataType\":{:?},", column.data_type)?;
        write!(file, "\"isNullable\":{:?},", column.is_nullable)?;
        write!(file, "\"isPrimaryKey\":{:?}", column.is_identifier)?;
        write!(file, "\"foreignTableName\":{:?},", column.reference_container_name)?;
        write!(file, "\"foreignColumnName\":{:?}", column.reference_property_name)?;
        write!(file, "}},")?;
    }
    write!(file, "]")?;
    write!(file, "}}")?;

    Ok(()) 
}