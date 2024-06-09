use std::{fs::File, io::Read, path::MAIN_SEPARATOR_STR};

use crate::{constants::TEMP_DIRECTORY, models::data_definitions::DataDefinition, DMResult};

pub fn parse_definitions(container: String) -> DMResult<DataDefinition> {
    let mut file = File::open(format!("{}{}{}{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR_STR, "definition_", container, ".json"))?;
    
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    
    Ok(serde_json::from_str(&json_data)?)
}