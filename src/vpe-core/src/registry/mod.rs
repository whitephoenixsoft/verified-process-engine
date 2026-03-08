use serde_json::Value;
use super::Guard;
use super::ContextMap;


pub struct VpeEvent {
    pub action: String,
    pub timestamp: u64, // Unix epoch
}


use std::collections::HashMap;
use std::sync::Arc;

type GuardFactory = Box<dyn Fn(Value) -> Result<Box<dyn Guard>, String> + Send + Sync>;

pub struct GuardRegistry {
    factories: HashMap<String, GuardFactory>,
}

impl GuardRegistry {
    pub fn new() -> Self {
        let mut registry = Self { factories: HahMap::new() };

        registry.register("Equals", |params| { 
            let path = params["path"].as_str().ok_or("Missing path")?.to_string();
            let expected = Value::from_json(params["value"].clone());
            Ok(Box::new(EqualsGuard { path, expected }))
        });

        registry
    }

    pub fn register<F>(&mut self, id: &str, factory: F) where F: Fn(Value) -> Result<Box<dyn Guard>, String> + Send + Sync + 'static {
        self.factories.insert(id.to_string, Box::new(factory));
    }

    pub fn create_guard(&self, id: &str, params: Value) -> Result<Box<dyn Guard>, String> {
        let factory = self.factories.get(id)
            .ok_or_else(|| format!("Unknown guard type: {}", id))?;
        factory(params)
    }
}
