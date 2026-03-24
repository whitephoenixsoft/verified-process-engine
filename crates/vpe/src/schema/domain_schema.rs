use crate::schema::types::FieldDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSchema {
    pub domain: String,
    pub version: String,
    pub fields: Vec<FieldDefinition>,
}
