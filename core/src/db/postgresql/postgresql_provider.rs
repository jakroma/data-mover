use std::path::Path;

use diesel::{Table};
use tokio_postgres::Row;

use crate::{
    constants::postgresql::{POSTGRESQL_COLUMNS_QUERY, POSTGRESQL_SCHEMA_TABLE}, db::postgresql::postgresql_data_types::{get_data_as_type, PostgresDataType}, models::data_definitions::{DataDefinition, DataPropertyInfo}, writers::data_writer::write_data, DMResult
};

use super::postgresql::Postgresql;

impl Postgresql {
    pub async fn create_table_info(&self) -> DMResult<Vec<DataDefinition>> {
        let rows: Vec<tokio_postgres::Row> =
            self.client.query(POSTGRESQL_SCHEMA_TABLE, &[]).await?;
        let tables: Vec<String> = get_tables_with_schema(rows);
        let mut result: Vec<DataDefinition> = Vec::new();
        for table in tables {
            let columns: Vec<DataPropertyInfo> = self.create_column_info(&table).await?;
            let table_info: DataDefinition = DataDefinition {
                data_container_name: table,
                properties_info: columns,
            };

            result.push(table_info);
        }
        Ok(result)
    }

    pub async fn create_column_info(&self, table_name: &str) -> DMResult<Vec<DataPropertyInfo>> {
        let column_info: Vec<tokio_postgres::Row> = self
            .client
            .query(POSTGRESQL_COLUMNS_QUERY, &[&table_name])
            .await?;

        let mut result: Vec<DataPropertyInfo> = Vec::new();
        column_info.iter().for_each(|row: &tokio_postgres::Row| {
            result.push(DataPropertyInfo {
                property_name: row.get(2),
                data_type: row.get(3),
                is_nullable: row.get(4),
                is_identifier: row.get(5),
                reference_container_name: todo!(),
                reference_property_name: todo!(),
            })
        });

        Ok(result)
    }


    pub async fn create_data_dump(
        &self,
        data_definition: &DataDefinition,
        file_path: String,
    ) -> DMResult<()> {
        let column_elements = data_definition.properties_info.len();

        let column_names: Vec<String> = data_definition.properties_info.iter()
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
                    let data: PostgresDataType = get_data_as_type(&row, index)?;
                    write_data(data, Path::new(&file_path), &column_elements, &index)?;
                }
            }

            offset += limit;
        }

        Ok(())
    }
}

fn get_tables_with_schema(rows: Vec<tokio_postgres::Row>) -> Vec<String> {
    let tables: Vec<String> = rows
        .iter()
        .map(|row: &tokio_postgres::Row| {
            let schema: String = row.get(0);
            let table_name: String = row.get(1);
            format!("{0}.{1}", schema, table_name)
        })
        .collect();
    tables
}
