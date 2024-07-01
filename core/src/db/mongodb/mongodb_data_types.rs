pub fn get_bson_type(property_type: &str) -> &str {
    let result = match property_type {
        "integer" => "int",
        "character varying" => "string",
        "timestamp without time zone" => "date",
        _ => "string",
    };

    result
}