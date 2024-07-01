use std::hash::RandomState;

use async_trait::async_trait;
use log::info;
use mongodb::bson::{self, doc, Bson};

use crate::db::mongodb::mongodb_data_types::get_bson_type;
use crate::models::data_definitions::{DataPropertyInfo};
use crate::parsers::parse_data;
use crate::{
    migrator::migrator_steps::DataReceiver, parsers::parse_definitions,
    utils::file_utils::load_sorted_containers_from_file, DMResult,
};

use super::mongodb::Mongodb;

#[async_trait]
impl DataReceiver for Mongodb {
    async fn receive_data(&self) -> DMResult<()> {
        let migration_containers = load_sorted_containers_from_file()?;

        for container in migration_containers {
            let definition = parse_definitions(&container)?;
            info!("[Migration][{}] Read definitions", definition.container_name);
            let validation_schema = generate_validation_schema(definition.properties_info);
            info!("[Migration][{}] validation schema created {:?}", definition.container_name, validation_schema);

            let byte_data = parse_data(&container);
        }

        Ok(())
    }
}

fn generate_validation_schema(properties: Vec<DataPropertyInfo>) -> bson::Document {
    let mut required_fields = Vec::new();
    let mut properties_doc = doc! {};

    for prop in properties {
        let bson_type = get_bson_type(&prop.data_type.as_str());

        if !prop.is_nullable {
            required_fields.push(prop.property_name.clone());
        }

        let mut property_doc = doc! {
            "bsonType": bson_type,
        };

        property_doc.insert(
            "description",
            format!(
                "must be a {}{}",
                bson_type,
                if !prop.is_nullable {
                    " and is required"
                } else {
                    ""
                }
            ),
        );

        properties_doc.insert(prop.property_name, property_doc);
    }

    doc! {
        "bsonType": "object",
        "required": Bson::Array(required_fields.into_iter().map(Bson::String).collect()),
        "properties": properties_doc
    }
}
