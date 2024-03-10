use chrono::NaiveDate;
use tokio_postgres::{Row, types::{Type}};

use crate::DMResult;

impl PostgresDataType {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            PostgresDataType::Int(i) => format!("{}", i).into_bytes(),
            PostgresDataType::Float(f) => format!("{}", f).into_bytes(),
            PostgresDataType::Text(s) => format!("\"{}\"", s).into_bytes(),
            PostgresDataType::Varchar(v) => format!("\"{}\"", v).into_bytes(),
            PostgresDataType::Date(d) => d.to_string().into_bytes(),
        }
    }
}

pub enum PostgresDataType {
    Int(i32),
    Float(f32),
    Date(NaiveDate),
    Text(String),
    Varchar(String),
}

pub fn get_data_as_type(row: &Row, index: usize) -> DMResult<PostgresDataType> {
    let column_type = row.columns()[index].type_();
    match *column_type {
        Type::INT4 => Ok(PostgresDataType::Int(row.try_get(index)?)),
        Type::FLOAT4 => Ok(PostgresDataType::Float(row.try_get(index)?)),
        Type::DATE => Ok(PostgresDataType::Date(row.try_get(index)?)),
        Type::TEXT => Ok(PostgresDataType::Text(row.try_get(index)?)),
        Type::VARCHAR => Ok(PostgresDataType::Text(row.try_get(index)?)),
        _ => Err(crate::DMError::NotSupportedType())
    }
}