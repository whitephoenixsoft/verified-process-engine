
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
    pub fn resolve_field(&self, path: &str) -> Option<FieldDefinitionRef<'_>> {
        let (namespace, field_name) = path.split_once('.')?;

        match namespace {
            "rec" => self
                .namespaces
                .rec
                .iter()
                .find(|f| f.name == field_name)
                .map(FieldDefinitionRef::User),
            "ext" => self
                .namespaces
                .ext
                .iter()
                .find(|f| f.name == field_name)
                .map(FieldDefinitionRef::User),
            "calc" => self
                .namespaces
                .calc
                .iter()
                .find(|f| f.name == field_name)
                .map(FieldDefinitionRef::User),
            "sys" => resolve_system_field(field_name).map(FieldDefinitionRef::System),
            _ => None,
        }
    }

    pub fn resolve_path_type(&self, path: &str) -> Option<&SchemaFieldType> {
        self.resolve_field(path).map(|f| f.field_type()).clone()
    }
}

#[derive(Debug, Clone)]
pub enum FieldDefinitionRef<'a> {
    User(&'a FieldDefinition),
    System(SystemFieldDefinition),
}

impl<'a> FieldDefinitionRef<'a> {
    pub fn field_type(&self) -> &SchemaFieldType {
        match self {
            FieldDefinitionRef::User(field) => &field.field_type,
            FieldDefinitionRef::System(field) => &field.field_type,
        }
    }

    pub fn enum_values(&self) -> Option<&[String]> {
        match self {
            FieldDefinitionRef::User(field) => field.enum_values.as_deref(),
            FieldDefinitionRef::System(field) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemFieldDefinition {
    pub field_type: SchemaFieldType,
}

fn resolve_system_field(field_name: &str) -> Option<SystemFieldDefinition> {
    match field_name {
        "now" => Some(SystemFieldDefinition {
            field_type: SchemaFieldType::DateTime,
        }),
        "trace_id" => Some(SystemFieldDefinition {
            field_type: SchemaFieldType::String,
        }),
        _ => None,
    }
}