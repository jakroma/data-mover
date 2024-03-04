use crate::{constants::postgresql::{DEFAULT_SCHEMA, POSTGRESQL_COLUMNS_QUERY, POSTGRESQL_SCHEMA_TABLE}, DMResult};

use super::postgresql::{ColumnInfo, Postgresql, TableInfo};

impl Postgresql {
    pub async fn create_table_info(&self) -> DMResult<Vec<TableInfo>> {
        let rows: Vec<tokio_postgres::Row> =
            self.client.query(POSTGRESQL_SCHEMA_TABLE, &[]).await?;
        let tables: Result<Vec<String>, _> =
            rows.iter().map(|row| row.try_get::<_, String>(0)).collect();
        let mut result = Vec::new();
        for table in tables {
            for table_name in table {
                let columns = self.create_column_info(&table_name).await?;
                let table_info = TableInfo {
                    table_name,
                    table_schema: DEFAULT_SCHEMA.to_owned(),
                    columns,
                };

                result.push(table_info);
            }
        }
        Ok(result)
    }

    pub async fn create_column_info(
        &self,
        table_name: &str,
    ) -> DMResult<Vec<ColumnInfo>> {
        let column_info = self
            .client
            .query(POSTGRESQL_COLUMNS_QUERY, &[&table_name])
            .await?;

        let mut result: Vec<ColumnInfo> = Vec::new();
        column_info.iter().for_each(|row: &tokio_postgres::Row| {
            result.push(ColumnInfo {
                column_name: row.get(2),
                data_type: row.get(3),
                is_nullable: row.get(4),
                is_primary_key: row.get(5),
                foreign_table_name: todo!(),
                foreign_column_name: todo!(),
            })
        });

        Ok(result)
    }
}