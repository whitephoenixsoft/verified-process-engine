use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VpeEventKind {
    StateTransition,
    Migration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpeEvent {
    pub event_id: String,
    pub parent_event_id: Option<String>,
    pub trace_id: String,
    pub timestamp: i64,
    pub event_kind: VpeEventKind,
    pub action: String,
    pub state_before: String,
    pub state_after: String,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleView {
    pub anchor: VpeEvent,
    pub events: Vec<VpeEvent>,
}

impl ChronicleView {
    pub fn anchor(&self) -> &VpeEvent {
        &self.anchor
    }

    pub fn events(&self) -> &[VpeEvent] {
        &self.events
    }
}
