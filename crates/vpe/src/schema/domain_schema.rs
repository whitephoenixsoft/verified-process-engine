
use crate::schema::types::{FieldDefinition, SchemaFieldType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSchema {
    pub domain: String,
    pub version: String,
    pub namespaces: SchemaNamespaces,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaNamespaces {
    #[serde(default)]
    pub rec: Vec<FieldDefinition>,
    #[serde(default)]
    pub ext: Vec<FieldDefinition>,
    #[serde(default)]
    pub calc: Vec<FieldDefinition>,
}

impl DomainSchema {
    pub fn resolve_path_type(&self, path: &str) -> Option<&SchemaFieldType> {
        let (namespace, field_name) = path.split_once('.')?;

        match namespace {
            "rec" => self
                .namespaces
                .rec
                .iter()
                .find(|f| f.name == field_name)
                .map(|f| &f.field_type),
            "ext" => self
                .namespaces
                .ext
                .iter()
                .find(|f| f.name == field_name)
                .map(|f| &f.field_type),
            "calc" => self
                .namespaces
                .calc
                .iter()
                .find(|f| f.name == field_name)
                .map(|f| &f.field_type),
            "sys" => resolve_system_path_type(field_name),
            _ => None,
        }
    }
}

fn resolve_system_path_type(field_name: &str) -> Option<&'static SchemaFieldType> {
    match field_name {
        "now" => Some(&SchemaFieldType::DateTime),
        "trace_id" => Some(&SchemaFieldType::String),
        _ => None,
    }
}