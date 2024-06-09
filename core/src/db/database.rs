use std::collections::{HashMap, HashSet};

use async_trait::async_trait;

use crate::{migrator::migrator_steps::{DataProvider, DataReceiver, MigrationProviderResult, MigrationProviderStats}, models::{connection::MigrationConnection, migration_execution_settings::MigrationExecutionSettings, migration_setting::MigrationSetting}, DMError, DMResult};

use super::{mongodb::mongodb::Mongodb, postgresql::postgresql::Postgresql};

#[derive(Clone)]
pub enum DatabaseType {
    Postgresql,
    Mongodb
}

pub enum DatabaseConnectionType {
    Postgresql(Postgresql),
    Mongodb(Mongodb),
}

impl DatabaseConnectionType {
    pub fn new(s: &String) -> DMResult<DatabaseType> {
        if s.starts_with("postgresql") {
            Ok(DatabaseType::Postgresql)
        }
        else if s.starts_with("mongodb") {
            Ok(DatabaseType::Mongodb)
        }
        else {
            Err(DMError::NotSupportedDb())
        }
    }
}

pub async fn create_connection(connection_model: MigrationConnection, settings: &MigrationSetting) -> DMResult<DatabaseConnectionType> {
    match connection_model.db_type {
        DatabaseType::Postgresql => {
            let conn = Postgresql::new(connection_model, MigrationExecutionSettings::new(settings.threads_limit, settings.pagination_limit)).await?;
            Ok(DatabaseConnectionType::Postgresql(conn))
        }
        DatabaseType::Mongodb => {
            let conn = Mongodb::new(connection_model, MigrationExecutionSettings::new(settings.threads_limit, settings.pagination_limit)).await?;
            Ok(DatabaseConnectionType::Mongodb(conn))
        }
    }
}

#[async_trait]
impl DataProvider for DatabaseConnectionType {
    async fn get_data(&self, stats: MigrationProviderStats) -> DMResult<MigrationProviderResult> {
        match self {
            DatabaseConnectionType::Postgresql(p) => p.get_data(stats).await,
            DatabaseConnectionType::Mongodb(m) => m.get_data(stats).await,
        }
    }
}

#[async_trait]
impl DataReceiver for DatabaseConnectionType {
    async fn receive_data(&self) -> DMResult<()> {
        match self {
            DatabaseConnectionType::Postgresql(p) => p.receive_data().await,
            DatabaseConnectionType::Mongodb(m) => m.receive_data().await,
        }
    }
}

pub fn sort_tables_by_dependencies(order_hash: &HashMap<String, Vec<String>>) -> DMResult<Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut tables: HashSet<String> = HashSet::new();

    for (table, dependencies) in order_hash {
        for dependency in dependencies {
            graph.entry(dependency.clone())
                .or_insert_with(Vec::new)
                .push(table.clone());
            *in_degree.entry(table.clone()).or_insert(0) += 1;
            tables.insert(dependency.clone());
            tables.insert(table.clone());
        }
    }

    let mut queue: Vec<String> = tables.iter()
        .filter(|&table| *in_degree.get(table).unwrap_or(&0) == 0)
        .cloned()
        .collect();

    let mut sorted_tables: Vec<String> = Vec::new();

    while let Some(table) = queue.pop() {
        sorted_tables.push(table.clone());
        if let Some(dependents) = graph.get(&table) {
            for dependent in dependents {
                if let Some(degree) = in_degree.get_mut(dependent) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(dependent.clone());
                    }
                }
            }
        }
    }

    if sorted_tables.len() != tables.len() {
        return Err(DMError::Error("Cycle detected in table dependencies".to_string()));
    }

    Ok(sorted_tables)
}