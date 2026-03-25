use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawSource {
    pub domain: String,
    pub process: String,
    pub version: String,
    pub initial_state: String,
    pub states: Vec<StateSource>,
    #[serde(default)]
    pub migration_rules: Vec<MigrationRuleSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSource {
    pub name: String,
    #[serde(default)]
    pub is_transient: bool,
    #[serde(default)]
    pub transitions: Vec<TransitionSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSource {
    pub action: String,
    pub to: String,
    #[serde(default)]
    pub priority: u32,
    #[serde(default)]
    pub guards: Vec<GuardSource>,
    #[serde(default)]
    pub effects: Vec<EffectSource>,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardSource {
    #[serde(rename = "type")]
    pub guard_type: String,
    #[serde(flatten)]
    pub params: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSource {
    #[serde(rename = "type")]
    pub effect_type: String,
    pub target: Option<String>,
    pub action: Option<String>,
    #[serde(default)]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRuleSource {
    pub from_state: String,
    pub to_state: String,
}
