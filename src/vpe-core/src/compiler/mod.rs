pub mod domain;
pub mod namespace;

pub enum DataType { String, Number, Bool }

impl DataType {
    /// Returns true if the JSON value matches the expected schema type
    pub fn matches(&self, value: &serde_json::Value) -> bool {
        match self {
            DataType::String => value.is_string(),
            DataType::Number => value.is_number(),
            DataType::Bool => value.is_boolean(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DataType::String => "String",
            DataType::Number => "Number",
            DataType::Bool => "Boolean",
        }
    }
}

pub struct VpeDag {
    pub domain: String,
    pub version: String,
    pub initial_state_idx: usize,
    pub nodes: Vec<Node>, // Array-backed graph
}

pub struct Node {
    pub name: String,
    pub transitions: Vec<Edge>,
}
    
pub struct Edge {
    pub action: String,
    pub priority: u32,
    pub target_idx: usize,
    pub guards: Vec<Box<dyn Guard>>,
    pub effects: Vec<String>,
}

pub struct StateManifest {
    /// Union of all requirements for all guards in this state.
    pub required_history: Vec<HistoryRequirement>,
}

use std::collections::{HashMap, HashSet, VecDeque};
use sha2::{Sha256, Digest};

pub struct VpeCompiler<'a> {
    registry: Arc<GuardRegistry>,
    schema: &'a SchemaDefinition,
}

impl<'a> VpeCompiler<'a> {
    pub fn new(registry: Arc<GuardRegistry>, schema: &'a SchemaDefinition) -> Self {
        Self { registry, schema }
    }

    pub fn compile_and_validate(&self, json: &str) -> Result<RegistrationReport, VpeError> {
        // 1. PHASE 1: Ingestion (Parse & Bind to Schema)
        let raw_dag: RawDag = serde_json::from_str(json).map_err(VpeError::ParseError)?;
        self.validate_schema_binding(&raw_dag)?;

        // 2. PHASE 2 & 3: Build Internal DAG & Audit Topology
        let dag = self.build_dag(raw_dag)?;
        self.audit_topology(&dag)?;

        // 3. PHASE 4: Saga & Side-Effect Safety
        self.audit_sagas(&dag)?;

        // 4. PHASE 5: Manifest Synthesis
        let manifests = self.synthesize_manifests(&dag);

        // 5. GENERATE DIGEST (Hash of the DAG structure)
        let digest = self.calculate_digest(&dag);

        Ok(RegistrationReport {
            domain: dag.domain.clone(),
            version: dag.version.clone(),
            digest,
            manifests,
            warnings: vec![], // Add warnings gathered during passes
        })
    }

    /// PHASE 2: Check for Orphans and Auto-Loops
    fn audit_topology(&self, dag: &VpeDag) -> Result<(), VpeError> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(dag.initial_state_idx);

        // Reachability (Orphan Check)
        while let Some(idx) = queue.pop_front() {
            if visited.insert(idx) {
                for transition in &dag.nodes[idx].transitions {
                    queue.push_back(transition.target_idx);
                }
            }
        }

        if visited.len() < dag.nodes.len() {
            return Err(VpeError::CompilerError("Orphan states detected.".into()));
        }

        // Auto-Loop Detection (DFS for back-edges on null actions)
        self.check_for_auto_cycles(dag)
    }

    fn check_for_auto_cycles(&self, dag: &VpeDag) -> Result<(), VpeError> {
        for start_node in 0..dag.nodes.len() {
            let mut stack = vec![(start_node, HashSet::new())];
            while let Some((current, mut path)) = stack.pop() {
                if !path.insert(current) {
                    return Err(VpeError::CompilerError(format!(
                        "Infinite Auto-Loop in state: {}", dag.nodes[current].name
                    )));
                }

                for edge in &dag.nodes[current].transitions {
                    if edge.action.is_none() { // Only follow AUTO_TICK paths
                        stack.push((edge.target_idx, path.clone()));
                    }
                }
            }
        }
        Ok(())
    }

    /// PHASE 4: Saga Audit
    fn audit_sagas(&self, dag: &VpeDag) -> Result<(), VpeError> {
        for node in &dag.nodes {
            for edge in &node.transitions {
                if !edge.effects.is_empty() {
                    let target = &dag.nodes[edge.target_idx];
                    if !target.is_transient {
                        return Err(VpeError::CompilerError(format!(
                            "Action '{}' has side-effects but lands in stable state '{}'",
                            edge.action.as_deref().unwrap_or("auto"), target.name
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    /// PHASE 5: Manifest Synthesis
    fn synthesize_manifests(&self, dag: &VpeDag) -> HashMap<String, Vec<HistoryRequirement>> {
        let mut report = HashMap::new();

        for node in &dag.nodes {
            let mut reqs = HashSet::new();
            reqs.insert(HistoryRequirement::LastTransition); // Always required

            for trans in &node.transitions {
                for guard_def in &trans.guards {
                    // Pull hints from the Registry
                    if let Some(guard) = self.registry.get(&guard_def.guard_type) {
                        for req in guard.get_requirements() {
                            reqs.insert(req);
                        }
                    }
                }
            }
            report.insert(node.name.clone(), reqs.into_iter().collect());
        }
        report
    }

    fn calculate_digest(&self, dag: &VpeDag) -> String {
        let mut hasher = Sha256::new();
        // Hash the serialized DAG structure to ensure logic changes trigger a new hash
        hasher.update(serde_json::to_string(dag).unwrap());
        format!("{:x}", hasher.finalize())
    }

    fn build_manifest(state: &RawState) -> StateManifest {
        let mut requirements = HashSet::new();
        requirements.insert(HistoryRequirement::LastTransition); // Global Invariant
    
        for transition in &state.transitions {
            for guard in &transition.guards {
                for req in guard.get_requirements() {
                    requirements.insert(req);
                }
            }
        }
        StateManifest { required_history: requirements.into_iter().collect() }
    }
    
    //for migrations
    pub fn compile_and_validate(&mut self, json: &str) -> Result<VpeDag, String> {
        let raw: RawVpeJson = serde_json::from_str(json).map_err(|e| e.to_string())?;

        // PHASE 1: Build the Graph
        let mut dag = self.build_dag(&raw)?;

        // PHASE 2: Audit Migrations
        for rule in &raw.migrations {
            // 1. Verify Destination State
            if !dag.state_exists(&rule.to_state) {
                return Err(format!(
                    "Migration Error: Destination state '{}' does not exist in version {}.",
                    rule.to_state, raw.version
                ));
            }

            // 2. Verify Transforms
            for op in &rule.transforms {
                match op {
                    TransformOp::Move { from: _, to } | TransformOp::Set { target: to, .. } => {
                        // Re-use our "Naming Police" and "Schema Check"
                        self.validate_identifier_structure(to)?;
                        self.validate_write_path(to, &raw.domain)?; 
                        // Ensures 'to' is in the DomainSchema and NOT sys.*
                    },
                    _ => {}
                }
            }

            // 3. Verify Migration Guards
            for guard in &rule.migration_guards {
                let path = guard["path"].as_str().ok_or("Missing path in migration guard")?;
                self.validate_path(path, &raw.domain, false)?; // Read-only check
                self.audit_type(path, &guard["value"], &raw.domain)?; // Type check
            }
        }

        Ok(dag)
    }

    pub fn new(registry: Arc<GuardRegistry>) -> Self {
        Self { registry }
    }

    //regular compiling
    pub fn compile(&self, json_str: &str) -> Result<VpeDag, String> {
        // 1. Parse JSON into temporary raw structs
        let raw: RawProcess = serde_json::from_str(json_str)
            .map_err(|e| format!("JSON Parse Error: {}", e))?;

        // 2. Build Name-to-Index Map
        let mut name_to_idx = HashMap::new();
        for (idx, state) in raw.states.iter().enumerate() {
            name_to_idx.insert(state.name.clone(), idx);
        }

        // 3. Find the Initial State Index
        let initial_idx = *name_to_idx.get(&raw.initial_state)
            .ok_or_else(|| format!("Initial state '{}' not found", raw.initial_state))?;

        // 4. Inflate Nodes and Edges
        let mut nodes = Vec::new();
        for raw_state in raw.states {
            let mut transitions = Vec::new();

            for raw_edge in raw_state.transitions {
                let target_idx = *name_to_idx.get(&raw_edge.to)
                    .ok_or_else(|| format!("Target state '{}' not found", raw_edge.to))?;

                // HYDRATION: Call Registry to turn JSON into Rust Code
                let mut guards = Vec::new();
                for g_config in raw_edge.guards {
                    let guard_type = g_config["type"].as_str()
                        .ok_or("Guard missing 'type' field")?;
                    
                    // The Registry creates the Box<dyn Guard> here
                    let guard = self.registry.create_guard(guard_type, g_config)?;
                    guards.push(guard);
                }

                transitions.push(Edge {
                    action: raw_edge.action,
                    target_idx,
                    priority: raw_edge.priority,
                    guards,
                });
            }

            nodes.push(Node {
                name: raw_state.name,
                transitions,
            });
        }

        Ok(VpeDag {
            domain: raw.domain,
            version: raw.version,
            initial_state_idx: initial_idx,
            nodes,
        })
    }
    
    //for names and types specified
    pub fn compile_edge(&self, raw_edge: &RawEdge, domain: &str) -> Result<Edge, String> {
        // 1. Validate Identifier Characters & Structure
        for guard in &raw_edge.guards {
            let path = guard["path"].as_str().ok_or("Missing path")?;
            
            // NEW: The Identifier "Police"
            self.validate_identifier_structure(path)?;
            
            // 2. Hybrid Namespace Check (Enum + Schema)
            let dtype = self.validate_path(path, domain, false)?;
            
            // 3. Type Audit (Ensuring Value matches Schema Type)
            self.audit_type(path, &guard["value"], domain)?;
        }

        // ... continue to build the Edge ...
    }
    
    /// Validates that a string is a valid identifier (alphanumeric + underscores)
    fn is_valid_identifier(s: &str) -> bool {
        !s.is_empty() && 
        s.chars().all(|c| c.is_alphanumeric() || c == '_') &&
        !s.starts_with(|c: char| c.is_ascii_digit()) // Cannot start with a number
    }

    fn validate_identifier_structure(&self, full_path: &str) -> Result<(), String> {
        let parts: Vec<&str> = full_path.split('.').collect();
        
        // Rule: Must have at least Namespace.Field
        if parts.len() < 2 {
            return Err(format!("Identifier '{}' must use dot-notation (e.g., rec.id)", full_path));
        }

        // Rule: Every segment must be a valid identifier
        for part in parts {
            if !Self::is_valid_identifier(part) {
                return Err(format!(
                    "Invalid Identifier segment '{}' in path '{}'. Only alphanumeric and underscores allowed.", 
                    part, full_path
                ));
            }
        }
        Ok(())
    }

    fn validate_path(&self, full_path: &str, domain_name: &str, is_write: bool) -> Result<DataType, String> {
        // 1. Split the path (e.g., "rec.order.id" -> ["rec", "order.id"])
        let (prefix, suffix) = full_path.split_once('.')
            .ok_or_else(|| format!("Path '{}' must include a namespace.", full_path))?;

        // 2. Resolve the category via the Enum
        let category = NamespaceCategory::from_prefix(prefix)?;

        // 3. APPLY ENFORCEMENT RULES
        match category {
            NamespaceCategory::System => {
                if is_write {
                    return Err("Security Violation: 'sys' namespace is read-only.".into());
                }
                // System types are "Trusted" - we can skip schema check or 
                // have a hardcoded SysSchema. Let's assume they are flexible for now.
                Ok(DataType::String) 
            },
            _ => {
                // 4. Check the Configurable Schema for rec, ext, or calc
                let schema = self.get_schema(domain_name)?;
                let fields = schema.registry.get(&category)
                    .ok_or_else(|| format!("Namespace '{}' not initialized for domain '{}'.", prefix, domain_name))?;

                fields.get(suffix)
                    .cloned()
                    .ok_or_else(|| format!("Field '{}' not found in namespace '{}'.", suffix, prefix))
            }
        }
    }

    //that names exist
    fn check_schema(&self, full_path: &str, domain: &str) -> Result<(), String> {
        let (ns, path) = full_path.split_once('.')
            .ok_or_else(|| format!("Invalid path format: {}", full_path))?;

        let schema = self.domain_schemas.get(domain)
            .ok_or_else(|| format!("No schema for domain {}", domain))?;

        if let Some(fields) = schema.namespaces.get(ns) {
            if fields.contains_key(path) {
                return Ok(());
            }
        }

        Err(format!("Path '{}.{}' not found in {} schema", ns, path, domain))
    }
    
    //check types in guards
    fn audit_type(&self, path: &str, json_value: &serde_json::Value, domain: &str) -> Result<(), String> {
        // 1. Skip validation for sys.* as they are internal and trusted
        if path.starts_with("sys.") {
            return Ok(());
        }

        // 2. Resolve the expected type from the schema
        let expected_type = self.get_type_from_schema(path, domain)?;

        // 3. Compare
        if !expected_type.matches(json_value) {
            return Err(format!(
                "Type Mismatch on '{}': The schema expects a {}, but the JSON provided a incompatible value: {}.",
                path, 
                expected_type.as_str(),
                json_value
            ));
        }

        Ok(())
    }

    fn check_for_infinite_auto_loops(&self, dag: &VpeDag) -> Result<(), String> {
        for (start_idx, _) in dag.nodes.iter().enumerate() {
            let mut visited = HashSet::new();
            let mut stack = vec![start_idx];

            while let Some(current) = stack.pop() {
                if !visited.insert(current) {
                    return Err(format!(
                        "Infinite Loop Detected: Auto-transitions lead back to state '{}'", 
                        dag.nodes[current].name
                    ));
                }

                // Only follow edges that don't require an external Action
                for edge in &dag.nodes[current].transitions {
                    if edge.action == "AUTO_TICK" || edge.action.is_empty() {
                        stack.push(edge.target_idx);
                    }
                }
            }
        }
        Ok(())
    }
    
    pub fn execute_full(...) {
        let mut ticks = 0;
        let max_ticks = 50; // No real-world workflow needs 50 auto-moves in one go
    
        loop {
            match runtime.evaluate(...) {
                Ok(verdict) => {
                    // ... update state ...
                    ticks += 1;
                    if ticks > max_ticks {
                        return Err("Runtime Error: Maximum automated transition depth exceeded.");
                    }
                }
                Err(_) => break, // Hit a state requiring an Action
            }
        }
    }
    
    fn audit_sagas(&self, dag: &VpeDag) -> Result<(), String> {
        for node in &dag.nodes {
            for trans in &node.transitions {
                if trans.has_external_effects() && !dag.is_transient_state(trans.to_idx) {
                    return Err(format!(
                        "Safety Violation: Action '{}' has external effects but lands in stable state '{}'. Use a Saga state.",
                        trans.action, dag.nodes[trans.to_idx].name
                    ));
                }
            }
        }
        Ok(())
    }

    //another way audit saga state
    pub fn audit_side_effects(&self, dag: &VpeDag) -> Result<(), String> {
        for node in &dag.nodes {
            for edge in &node.transitions {
                // If this transition triggers a Side-Effect (EffectCount > 0)
                if !edge.effects.is_empty() {
                    let target_node = &dag.nodes[edge.target_idx];
                    
                    // CRITICAL CHECK: Target must be transient
                    if !target_node.is_transient {
                        return Err(format!(
                            "Atomicity Risk: Action '{}' has effects but lands in stable state '{}'. \
                             Target must be marked 'is_transient: true'.",
                            edge.action, target_node.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    //uses schema definitions
    fn validate_guard_types(&self, guard_def: &RawGuard) -> Result<(), VpeError> {
        if guard_def.path.starts_with("rec.") {
            let expected_type = self.schema.resolve_rec_type(&guard_def.path)
                .ok_or(VpeError::CompilerError(format!(
                    "Field '{}' not found in Domain Schema", guard_def.path
                )))?;

            // Simple type matching logic
            match (expected_type, &guard_def.value) {
                (VpeType::Number, Value::Number(_)) => Ok(()),
                (VpeType::String, Value::String(_)) => Ok(()),
                (VpeType::Boolean, Value::Bool(_)) => Ok(()),
                _ => Err(VpeError::CompilerError(format!(
                    "Type mismatch for field '{}'. Expected {:?}.", 
                    guard_def.path, expected_type
                ))),
            }
        } else {
            // Handle sys.* and ext.* with internal engine defaults
            Ok(())
        }
    }
    
    // date vs duration 
    fn validate_temporal_comparison(&self, path: &str, value: &VpeValue) -> Result<(), VpeError> {
        let field_type = self.schema.resolve_rec_type(path)
            .ok_or(VpeError::FieldNotFound(path.to_string()))?;

        match (field_type, value) {
            (VpeType::DateTime, VpeValue::DateTime(_)) => Ok(()),
            (VpeType::DateTime, VpeValue::SysPlaceholder(s)) if s == "sys.now" => Ok(()),
            (VpeType::DateTime, _) => Err(VpeError::TypeMismatch("DateTime fields must be compared to DateTimes or sys.now".into())),
            _ => Ok(()),
        }
    }
}
