pub mod registry;

pub trait Guard: Send + Sync {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool;
}

pub struct VpeDag {
    pub nodes: Vec<Node>,
}

pub struct Edge {
    pub action: String,
    pub priority: u32,
    pub target_idx: usize,
    pub guards: Vec<Box<dyn Guard>>,
    pub effects: Vec<String>,
}

pub struct VpeEvent {
    pub timestamp: i64,
    pub action: String,
    pub was_successful: bool,
    pub state_before: String,
    pub state_after: String,
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


