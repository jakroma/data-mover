use std::{fs::File, io::{BufRead, BufReader, Read}, path::MAIN_SEPARATOR_STR, vec};

use log::info;

use crate::{constants::TEMP_DIRECTORY, models::data_definitions::DataDefinition, DMResult};

pub fn parse_definitions(container: &String) -> DMResult<DataDefinition> {
    let mut file = File::open(format!("{}{}{}{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR_STR, "definition_", container, ".json"))?;
    
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    
    Ok(serde_json::from_str(&json_data)?)
}

pub fn parse_data(container: &String) -> DMResult<Vec<Vec<Vec<u8>>>> {
    let path = format!("{}{}{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR_STR, container, "_data.dbc");
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut result: Vec<Vec<Vec<u8>>> = vec![];

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut current_field: Vec<u8> = Vec::new();
    let mut current_record: Vec<Vec<u8>> = Vec::new();
    let mut property_begin = 0;

    for byte in buffer {
        match byte {
            0x1f => {
                if property_begin > 0 {
                    if !current_record.is_empty() {
                        result.push(current_record.clone());
                        current_record.clear();
                    }
                }
                property_begin += 1;
            }
            0x1e => {
                if !current_field.is_empty() {
                    current_record.push(current_field.clone());
                    current_field.clear();
                }
            }
            _ => current_field.push(byte),
        }
    }

    if !current_record.is_empty() {
        result.push(current_record);
    }

    Ok(result)
}
