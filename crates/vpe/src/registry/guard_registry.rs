use crate::error::VpeError;
use crate::registry::Guard;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub type GuardFactory =
    Arc<dyn Fn(&Value) -> Result<Box<dyn Guard>, VpeError> + Send + Sync + 'static>;

#[derive(Clone, Default)]
pub struct GuardRegistry {
    factories: HashMap<String, GuardFactory>,
}

impl GuardRegistry {
    pub fn builder() -> GuardRegistryBuilder {
        GuardRegistryBuilder::new()
    }

    pub fn get(&self, id: &str) -> Option<&GuardFactory> {
        self.factories.get(id)
    }
}

pub struct GuardRegistryBuilder {
    factories: HashMap<String, GuardFactory>,
}

impl GuardRegistryBuilder {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn with_builtins(mut self) -> Self {
        crate::registry::builtins::register_builtins(&mut self);
        self
    }

    pub fn register_guard(&mut self, id: impl Into<String>, factory: GuardFactory) -> &mut Self {
        self.factories.insert(id.into(), factory);
        self
    }

    pub fn build(self) -> Result<GuardRegistry, VpeError> {
        Ok(GuardRegistry {
            factories: self.factories,
        })
    }
}
