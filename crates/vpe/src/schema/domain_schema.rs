use crate::schema::types::{FieldDefinition, SchemaFieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSchema {
    pub domain: String,
    pub version: String,
    pub fields: Vec<FieldDefinition>,
}

impl DomainSchema {
    pub fn resolve_path_type(&self, path: &str) -> Option<&SchemaFieldType> {
        let (namespace, field_name) = path.split_once('.')?;

        match namespace {
            "rec" | "ext" | "calc" => self
                .fields
                .iter()
                .find(|f| f.name == field_name)
                .map(|f| &f.field_type),
            "sys" => None,
            _ => None,
        }
    }
}
