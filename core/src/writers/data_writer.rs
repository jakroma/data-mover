use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::{
    db::postgresql::{postgresql::Postgresql, postgresql_data_types::PostgresDataType},
    DMResult,
};

pub fn write_data(
    data: PostgresDataType,
    file_path: &Path,
    data_count: &usize,
    data_element: &usize,
) -> DMResult<()> {
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    let first_element = &0;
    let mut data_elements = &data_element.clone();

    if data_elements == first_element {
        file.write(&[0x1f])?;
    }

    file.write(&[0x1f])?;
    file.write(&data.as_bytes())?;

    if *data_count == data_elements + 1 {
        write!(file, "),\n")?;
    }

    Ok(())
}
