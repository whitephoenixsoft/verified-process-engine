
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemaFieldType {
    String,
    Number,
    Boolean,
    DateTime,
    Duration,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: SchemaFieldType,
    pub description: Option<String>,
    #[serde(default)]
    pub enum_values: Option<Vec<String>>,
}