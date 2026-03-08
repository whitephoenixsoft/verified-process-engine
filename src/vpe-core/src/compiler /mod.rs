
#[derive(Debug, PartialEq)]
pub enum NamespaceCategory {
    System,   // sys.* (Hardcoded logic)
    Record,   // rec.* (Dynamic schema)
    External, // ext.* (Dynamic schema)
    Calc,     // calc.* (Dynamic schema)
}

impl NamespaceCategory {
    fn from_prefix(prefix: &str) -> Result<Self, String> {
        match prefix {
            "sys"  => Ok(NamespaceCategory::System),
            "rec"  => Ok(NamespaceCategory::Record),
            "ext"  => Ok(NamespaceCategory::External),
            "calc" => Ok(NamespaceCategory::Calc),
            _      => Err(format!("Unknown namespace prefix: '{}'", prefix)),
        }
    }
}

   

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

pub struct DomainSchema {
    pub name: String,
    // We only store definitions for non-system namespaces
    pub registry: HashMap<NamespaceCategory, HashMap<String, DataType>>,
}

impl DomainSchema {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            registry: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, category: NamespaceCategory, field: &str, dtype: DataType) -> Result<(), String> {
        if category == NamespaceCategory::System {
            return Err("Cannot add custom fields to the 'sys' namespace.".into());
        }
        
        self.registry
            .entry(category)
            .or_insert_with(HashMap::new())
            .insert(field.to_string(), dtype);
        Ok(())
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

pub struct VpeCompiler {
    registry: Arc<GuardRegistry>,
}

impl VpeCompiler {
    pub fn new(registry: Arc<GuardRegistry>) -> Self {
        Self { registry }
    }

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
}
