

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