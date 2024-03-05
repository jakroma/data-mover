use std::{fs, path::{self, Path}};

use crate::{constants::TEMP_DIRECTORY, db::postgresql::postgresql::Postgresql, writers::data_container_definition_writer::write_definition, DMResult};

pub trait DataProvider {
    async fn execute(&self) -> DMResult<()>;
}

fn prepare_temp_folder() -> DMResult<String> {
    let string_path = TEMP_DIRECTORY.join(path::MAIN_SEPARATOR_STR);
    let path = Path::new(&string_path);
    if !path.exists() {
        fs::create_dir(path)?;
    }

    Ok(string_path)
}

impl DataProvider for Postgresql {
    async fn execute(&self) -> DMResult<()> {
        let temp_folder_path = prepare_temp_folder()?;
        let table_infos = self.create_table_info().await?;

        for table_info in table_infos {

            let definition_path = format!("{0}{1}{2}_definition.json", temp_folder_path, path::MAIN_SEPARATOR, table_info.table_name);
            let data_path = format!("{0}{1}{2}_data.dbc", temp_folder_path, path::MAIN_SEPARATOR, table_info.table_name);
            
            write_definition(&table_info, Path::new(&definition_path))?;
        }

        Ok(())
    }
}