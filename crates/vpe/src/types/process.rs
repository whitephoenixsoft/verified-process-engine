use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub type ContextMap = HashMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessRef {
    pub domain: String,
    pub process: String,
    pub version: String,
}

impl ProcessRef {
    pub fn new(
        domain: impl Into<String>,
        process: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            process: process.into(),
            version: version.into(),
        }
    }
}
