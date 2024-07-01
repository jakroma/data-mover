use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataDefinition {
    pub container_name: String,
    pub properties_info: Vec<DataPropertyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPropertyInfo {
    pub property_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_identifier: bool,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub reference_container_name: Option<String>,
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    pub reference_property_name: Option<String>,
}

fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|x| !x.is_empty()))
}