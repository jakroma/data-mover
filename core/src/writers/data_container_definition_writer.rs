use std::{fs::File, path::Path};

use crate::{models::data_definitions::DataDefinition, DMResult};

pub fn write_definition(table_info: &DataDefinition, file_path: &Path) -> DMResult<()> {
    let mut file: File = File::create(file_path)?;
    serde_json::to_writer(file, table_info)?;

    Ok(())
}