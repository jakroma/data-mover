#[derive(Debug)]
pub struct DataDefinition {
    pub data_container_name: String,
    pub property_info: Vec<DataPropertyInfo>,
}

#[derive(Debug)]
pub struct DataPropertyInfo {
    pub property_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_identifier: bool,
    pub reference_container_name: String,
    pub reference_property_name: String
}