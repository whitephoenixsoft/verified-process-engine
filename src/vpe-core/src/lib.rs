pub mod registry;

use std::collections::HashMap;
use serde_json::Value;

/// The ContextMap is a flat dictionary of namespaced keys
pub type ContextMap = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HistoryRequirement {
    /// Always required for the "Anchor" invariant.
    LastTransition,
    /// Needs the most recent event of a specific type (e.g., "Payment_Captured").
    LastEventOfAction(String),
    /// Needs all events of a type within a sliding window (e.g., "Login_Failure" last 24h).
    EventsInWindow { action: String, duration_seconds: u64 },
    /// Needs the full history of a specific field (rare, but useful for trends).
    FieldTrajectory(String),
}

pub trait Guard: Send + Sync {
    /// The logic check (Runtime)
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool;

    /// The dependency declaration (Compiler)
    fn get_requirements(&self) -> Vec<HistoryRequirement> {
        // Default: Most guards only need the Anchor
        vec![HistoryRequirement::LastTransition]
    }
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

pub struct VpeRequest {
    pub domain: String,
    pub version: String,
    pub trace_id: String,
    pub current_state_idx: usize,
    pub action: String,
    pub context: ContextMap, // rec.*, ext.*, sys.*
    pub history: Vec<VpeEvent>,
}

pub struct VpeSnapshot {
    pub initial_context: ContextMap,
    pub history: Vec<VpeEvent>,
}

#[derive(Debug)]
pub enum VpeError {
    CompilerError(String),
    RuntimeError(String),
    ProcessNotFound,
    LockError,
    SchemaMismatch(String),
}

/*
pub struct VpeEngine {
    guards: Arc<GuardRegistry>,
    processes: Arc<ProcessStore>,
    compiler: VpeCompiler,
}
*/
pub struct VpeEngine {
    registry: Arc<GuardRegistry>,
    // Map: Domain -> Version -> Compiled DAG
    dags: RwLock<HashMap<String, HashMap<String, Arc<VpeDag>>>>,
    schemas: RwLock<HashMap<String, DomainSchema>>,
}

impl VpeEngine {
    pub fn new(registry: Arc<GuardRegistry>) -> Self {
        Self {
            registry,
            dags: RwLock::new(HashMap::new()),
            schemas: RwLock::new(HashMap::new()),
        }
    }

    /// Registers a new Law (JSON). 
    /// Runs the Two-Phase Compiler: Graph Construction -> Migration Audit.
    pub fn register_process(&self, json: &str) -> Result<(), VpeError> {
        let schemas = self.schemas.read().map_err(|_| VpeError::LockError)?;
        
        // Phase 1: Build & Validate Graph/Types
        let mut compiler = VpeCompiler::new(self.registry.clone(), &schemas);
        let dag = compiler.compile_and_validate(json)?;

        // Phase 2: Store the compiled DAG
        let mut dags = self.dags.write().map_err(|_| VpeError::LockError)?;
        dags.entry(dag.domain.clone())
            .or_insert_with(HashMap::new())
            .insert(dag.version.clone(), Arc::new(dag));

        Ok(())
    }

    /// The "Execution Pulse."
    /// Evaluates a transition and returns the verdict for the Host to execute.
    pub fn execute(&self, request: VpeRequest) -> Result<VpeVerdict, VpeError> {
        let dags = self.dags.read().map_err(|_| VpeError::LockError)?;
        
        // 1. Resolve the specific version of the law
        let dag = dags.get(&request.domain)
            .and_then(|versions| versions.get(&request.version))
            .ok_or(VpeError::ProcessNotFound)?;

        // 2. Prepare Context (Seed sys.* values)
        let mut context = request.context;
        context.insert("sys.now".to_string(), current_timestamp());
        context.insert("sys.trace_id".to_string(), request.trace_id.into());

        // 3. Evaluate via Runtime (Direct Index Lookup)
        VpeRuntime::evaluate(
            dag, 
            request.current_state_idx, 
            &request.action, 
            &context, 
            &request.history
        ).map_err(VpeError::RuntimeError)
    }

    /// The "Dry-Run" Module.
    /// Analyzes the impact of a version update using historical snapshots.
    pub fn simulate(&self, domain: &str, target_version: &str, data: VpeSnapshot) -> SimulationReport {
        let dags = self.dags.read().unwrap();
        
        // Find the "Candidate" Law
        let target_dag = match dags.get(domain).and_then(|v| v.get(target_version)) {
            Some(dag) => dag,
            None => return SimulationReport::error("Target version not found"),
        };

        // Run the Replay Logic
        SimulationEngine::replay_history(
            target_dag, 
            &data.history, 
            data.initial_context
        )
    }

    /// Internal helper to prepare the context with system-level seeds
    fn prepare_context(&self, mut user_context: ContextMap, trace_id: &str) -> ContextMap {
        // Inject mandatory system values that the Engine manages
        user_context.insert("sys.trace_id".to_string(), Value::String(trace_id.to_string()));
        
        // Note: Other sys.* values like sys.now are usually provided 
        // by the Host in the initial context_json payload.
        
        user_context
    }
}


