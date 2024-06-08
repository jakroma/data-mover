use std::path::{self, Path};

use async_trait::async_trait;
use log::{error, info};
use tokio_postgres::Row;

use crate::{
    constants::postgresql::{
        POSTGRESQL_COLUMNS_QUERY, POSTGRESQL_SCHEMA_TABLE, POSTGRESQL_SCHEMA_TABLE_WHERE,
    },
    db::postgresql::postgresql_data_types::{get_typed_data, PostgresDataType},
    migrator::migrator_steps::DataProvider,
    models::data_definitions::{DataDefinition, DataPropertyInfo},
    utils::file_utils::prepare_temp_folder,
    writers::{data_container_definition_writer::write_definition, data_writer::write_data},
    DMResult,
};

use super::postgresql::Postgresql;

#[async_trait]
impl DataProvider for Postgresql {
    async fn get_data(&self) -> DMResult<()> {
        let temp_folder_path = prepare_temp_folder()?;
        info!("[Migration] Create table infos");
        let table_infos = self.create_table_info().await?;

        for table_info in table_infos {
            let definition_path = format!(
                "{0}{1}definition_{2}.json",
                temp_folder_path,
                path::MAIN_SEPARATOR,
                table_info.data_container_name
            );
            let data_path = format!(
                "{0}{1}{2}_data.dbc",
                temp_folder_path,
                path::MAIN_SEPARATOR,
                table_info.data_container_name
            );

            info!("[Migration][{}] Write definitions", table_info.data_container_name);
            write_definition(&table_info, Path::new(&definition_path))?;

            self.create_data_migration(&table_info, data_path).await?;
        }

        Ok(())
    }
}

impl Postgresql {
    pub async fn create_table_info(&self) -> DMResult<Vec<DataDefinition>> {
        let mut result: Vec<DataDefinition> = Vec::new();
        let schemas = self.get_schemas().await?;
        let tables: Vec<(String, String)> = get_tables_with_schema(schemas);

        for table in tables {
            let columns: Vec<DataPropertyInfo> = self.create_column_info(&table).await?;
            let table_info: DataDefinition = DataDefinition {
                data_container_name: format!("{}.{}", table.0, table.1),
                properties_info: columns,
            };

            result.push(table_info);
        }

        Ok(result)
    }

    pub async fn create_column_info(
        &self,
        schema_table: &(String, String),
    ) -> DMResult<Vec<DataPropertyInfo>> {
        let column_info: Vec<tokio_postgres::Row> = self
            .client
            .query(
                POSTGRESQL_COLUMNS_QUERY,
                &[&schema_table.0, &schema_table.1],
            )
            .await?;

        let mut result: Vec<DataPropertyInfo> = Vec::new();
        column_info.iter().for_each(|row: &tokio_postgres::Row| {
            result.push(DataPropertyInfo {
                property_name: row.get(2),
                data_type: row.get(3),
                is_nullable: row.get(4),
                is_identifier: row.get(5),
                reference_container_name: row.get::<_, Option<String>>(7),
                reference_property_name: row.get::<_, Option<String>>(8),
            })
        });

        Ok(result)
    }

    pub async fn create_data_migration(
        &self,
        data_definition: &DataDefinition,
        file_path: String,
    ) -> DMResult<()> {
        let column_elements = data_definition.properties_info.len();

        let column_names: Vec<String> = data_definition
            .properties_info
            .iter()
            .map(|column_info| column_info.property_name.clone())
            .collect();
        let columns = column_names.join(",");

        let limit: i16 = 10;
        let mut offset: i16 = 0;
        let mut all_data_fetched = false;

        while !all_data_fetched {
            let table_name: String = data_definition.data_container_name.clone();

            let statement = format!(
                "SELECT {} FROM {} LIMIT {} OFFSET {}",
                columns, table_name, limit, offset
            );
            println!("{}", statement);
            let rows: Vec<Row> = self.client.query(&statement, &[]).await?;

            if rows.len() < limit.try_into().unwrap() {
                all_data_fetched = true;
            }

            for row in rows {
                for (index, _) in row.columns().iter().enumerate() {
                    match get_typed_data(&row, index) {
                        Ok(Some(data)) => {
                            if let Err(e) = write_data(data, Path::new(&file_path), &column_elements, &index) {
                                error!("Error writing data: {}", e);
                                return Err(e.into());
                            }
                        }
                        Ok(None) => {
                            if let Err(e) = write_data(PostgresDataType::Undefined, Path::new(&file_path), &column_elements, &index) {
                                error!("Error writing data: {}", e);
                                return Err(e.into());
                            }
                        }
                        Err(e) => {
                            error!("Error get data type: {}", e);
                            return Err(e.into());
                        }
                    }
                }
            }

            offset += limit;
        }

        Ok(())
    }

    async fn get_schemas(&self) -> Result<Vec<Row>, crate::DMError> {
        match &self.schema {
            None => {
                let schemas: Vec<tokio_postgres::Row> =
                    self.client.query(POSTGRESQL_SCHEMA_TABLE, &[]).await?;
                Ok(schemas)
            }
            Some(filter_schema) => {
                let schemas: Vec<tokio_postgres::Row> = self
                    .client
                    .query(POSTGRESQL_SCHEMA_TABLE_WHERE, &[&filter_schema])
                    .await?;
                Ok(schemas)
            }
        }
    }
}

fn get_tables_with_schema(rows: Vec<tokio_postgres::Row>) -> Vec<(String, String)> {
    let schema_table_name: Vec<(String, String)> = rows
        .iter()
        .map(|row: &tokio_postgres::Row| {
            let schema: String = row.get(0);
            let table_name: String = row.get(1);
            (schema, table_name)
        })
        .collect();
    schema_table_name
}
