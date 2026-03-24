use crate::types::{ChronicleView, ContextMap, ProcessRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpeRequest {
    pub process: ProcessRef,
    pub trace_id: String,
    pub now: i64,
    pub current_state: String,
    pub action: String,
    pub context: ContextMap,
    pub chronicle: ChronicleView,
}
