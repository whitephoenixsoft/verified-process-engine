use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaFieldType {
    String,
    Number,
    Boolean,
    DateTime,
    Duration,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: SchemaFieldType,
    pub description: Option<String>,
}
