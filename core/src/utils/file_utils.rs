use std::{
    fs::{self, File}, io::{Read, Write}, path::{Path, MAIN_SEPARATOR, MAIN_SEPARATOR_STR}
};

use log::info;

use crate::{
    constants::{MIGRATION_ORDER_FILE_NAME, TEMP_DIRECTORY}, models::data_definitions::DataDefinition, DMError, DMResult,
};

pub fn prepare_temp_folder() -> DMResult<String> {
    let string_path = TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR);
    let path = Path::new(&string_path);
    if !path.exists() {
        info!("[Migration] Create folder {}!", path.display());
        fs::create_dir(path)?;
    }

    Ok(string_path)
}

pub fn save_sorted_containers_to_file(sorted_containers: &[String]) -> DMResult<()> {
    let json_data = serde_json::to_string(sorted_containers)?;
    let mut file = File::create(format!("{}{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR_STR, MIGRATION_ORDER_FILE_NAME))?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_sorted_containers_from_file() -> DMResult<Vec<String>> {
    let json_data = std::fs::read_to_string(format!("{}{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR_STR, MIGRATION_ORDER_FILE_NAME))?;
    let sorted_containers = serde_json::from_str(&json_data)?;
    
    Ok(sorted_containers)
}