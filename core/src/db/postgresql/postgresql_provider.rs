use crate::{
    constants::postgresql::{POSTGRESQL_COLUMNS_QUERY, POSTGRESQL_SCHEMA_TABLE},
    models::data_definitions::{DataDefinition, DataPropertyInfo}, DMResult,
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
                property_info: columns,
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
