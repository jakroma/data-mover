use chrono::{NaiveDate, NaiveDateTime};
use tokio_postgres::{Row, types::Type};
use uuid::Uuid;

use crate::DMResult;

impl PostgresDataType {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            PostgresDataType::SmallInt(i) => i.to_string().into_bytes(),
            PostgresDataType::Int(i) => i.to_string().into_bytes(),
            PostgresDataType::Float(f) => f.to_string().into_bytes(),
            PostgresDataType::Text(s) => format!("\"{}\"", s).into_bytes(),
            PostgresDataType::Varchar(v) => format!("\"{}\"", v).into_bytes(),
            PostgresDataType::Date(d) => d.to_string().into_bytes(),
            PostgresDataType::Uuid(u) => u.to_string().into_bytes(),
            PostgresDataType::Timestamp(t) => t.to_string().into_bytes(),
            PostgresDataType::Undefined => vec![0],
        }
    }
}

pub enum PostgresDataType {
    SmallInt(i16),
    Int(i32),
    Float(f32),
    Date(NaiveDate),
    Text(String),
    Varchar(String),
    Uuid(Uuid),
    Timestamp(NaiveDateTime),
    Undefined,
}

pub fn get_typed_data(row: &Row, index: usize) -> DMResult<Option<PostgresDataType>> {
    let column_type = row.columns()[index].type_();
    let data = match *column_type {
        Type::INT2 => row.try_get::<_, Option<i16>>(index)?.map(PostgresDataType::SmallInt),
        Type::INT4 => row.try_get::<_, Option<i32>>(index)?.map(PostgresDataType::Int),
        Type::FLOAT4 => row.try_get::<_, Option<f32>>(index)?.map(PostgresDataType::Float),
        Type::DATE => row.try_get::<_, Option<NaiveDate>>(index)?.map(PostgresDataType::Date),
        Type::TEXT => row.try_get::<_, Option<String>>(index)?.map(PostgresDataType::Text),
        Type::VARCHAR => row.try_get::<_, Option<String>>(index)?.map(PostgresDataType::Varchar),
        Type::UUID => row.try_get::<_, Option<Uuid>>(index)?.map(PostgresDataType::Uuid),
        Type::TIMESTAMP => row.try_get::<_, Option<NaiveDateTime>>(index)?.map(PostgresDataType::Timestamp),
        _ => None,
    };
    Ok(data)
}