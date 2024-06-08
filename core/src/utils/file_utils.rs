use std::{
    fs,
    path::{Path, MAIN_SEPARATOR, MAIN_SEPARATOR_STR},
};

use log::info;

use crate::{
    constants::TEMP_DIRECTORY, models::data_definitions::DataDefinition, DMError, DMResult,
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

pub fn load_definitions<'a>() -> impl Iterator<Item = Result<DataDefinition, DMError>> + 'a {
    let string_path = format!("{}{}", TEMP_DIRECTORY.join(MAIN_SEPARATOR_STR), MAIN_SEPARATOR);
    let dir_path = Path::new(&string_path);

    let entries_iter = fs::read_dir(dir_path)
        .map_err(DMError::IoError)
        .into_iter()
        .flat_map(|result| result);

    entries_iter.filter_map(|entry| {
        entry.ok().and_then(|e| {
            let path = e.path();
            if path.is_file()
                && path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .map_or(false, |s| s.starts_with("definitions_"))
            {
                Some(
                    fs::read_to_string(&path)
                        .map_err(DMError::IoError)
                        .and_then(|content| {
                            serde_json::from_str(&content).map_err(|op| DMError::SerdeJson(op))
                        }),
                )
            } else {
                None
            }
        })
    })
}
