use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HistoryRequirement {
    LastTransition,
    LastEventOfAction(String),
    EventsInWindow { action: String, duration_seconds: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextRequirement {
    Field(String),
    SystemField(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardRequirements {
    pub history: Vec<HistoryRequirement>,
    pub context: Vec<ContextRequirement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateManifest {
    pub history_requirements: Vec<HistoryRequirement>,
    pub context_requirements: Vec<ContextRequirement>,
}
