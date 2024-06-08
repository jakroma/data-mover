use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataDefinition {
    pub data_container_name: String,
    pub properties_info: Vec<DataPropertyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPropertyInfo {
    pub property_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_identifier: bool,
    pub reference_container_name: Option<String>,
    pub reference_property_name: Option<String>
}