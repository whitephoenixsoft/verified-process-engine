pub mod registry;

use std::collections::HashMap;
use serde_json::Value;

/// The ContextMap is a flat dictionary of namespaced keys
pub type ContextMap = HashMap<String, Value>;

pub trait Guard: Send + Sync {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VpeEvent {
    pub trace_id: String,      // The "Thread" connecting all actions
    pub timestamp: i64,
    pub actor: String,         // Who (System/User/AI)?
    pub action: String,
    pub was_successful: bool,
    pub state_before: String,
    pub state_after: String,
        pub metadata: Value,       // Any extra context
}

pub struct VpeEngine {
    guards: Arc<GuardRegistry>,
    processes: Arc<ProcessStore>,
    compiler: VpeCompiler,
}

impl VpeEngine {
    pub fn register_process(&self, json: &str) -> Result<(), VpeError>;
    pub fn execute(&self, request: VpeRequest) -> Result<(), VpeError>;
    pub fn simulate(&self, domain: &str, target_version: &str, data: VpeSnapshot) -> SimulationReport;
}


