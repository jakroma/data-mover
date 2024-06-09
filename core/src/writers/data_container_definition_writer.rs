use std::{fs::File, io::Write, path::Path};

use crate::{models::data_definitions::DataDefinition, DMResult};

pub fn write_definition(table_info: &DataDefinition, file_path: &Path) -> DMResult<()> {
    let mut file = File::create(file_path)?;
    let mut id = 0;

    write!(file, "{{")?;
    write!(
        file,
        "\"containerName\":{:?},",
        table_info.container_name
    )?;
    write!(file, "\"properties\":[")?;
    for column in &table_info.properties_info {
        id = id + 1;
        write!(file, "{{")?;
        write!(file, "\"propertyName\":{:?},", column.property_name)?;
        write!(file, "\"dataType\":{:?},", column.data_type)?;
        write!(file, "\"isNullable\":{:?},", column.is_nullable)?;
        write!(file, "\"isIdentifier\":{:?},", column.is_identifier)?;
        write!(
            file,
            "\"referenceContainerName\":{},",
            write_option(&column.reference_container_name)
        )?;
        write!(
            file,
            "\"referencePropertyName\":{}",
            write_option(&column.reference_property_name)
        )?;
        write!(file, "}},")?;
    }
    write!(file, "]")?;
    write!(file, "}}")?;

    Ok(())
}

fn write_option(option_column: &Option<String>) -> String {
    match &option_column {
        Some(name) => format!("{:?}", name),
        None => "\"\x00\"".to_string(),
    }
}