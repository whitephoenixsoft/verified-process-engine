use crate::types::{ContextMap, ProcessRef, VpeEventKind};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpeEffect {
    pub effect_type: String,
    pub target: Option<String>,
    pub action: Option<String>,
    pub params: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedEvent {
    pub event_kind: VpeEventKind,
    pub action: String,
    pub state_before: String,
    pub state_after: String,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpeVerdict {
    pub process: ProcessRef,
    pub trace_id: String,
    pub previous_state: String,
    pub next_state: String,
    pub state_patch: ContextMap,
    pub effects: Vec<VpeEffect>,
    pub emitted_events: Vec<PlannedEvent>,
}
