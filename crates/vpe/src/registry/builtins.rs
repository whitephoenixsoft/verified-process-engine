use crate::error::VpeError;
use crate::registry::guard_registry::{GuardFactory, GuardRegistryBuilder};
use crate::registry::Guard;
use crate::types::ContextMap;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug)]
struct DefaultGuard;

impl Guard for DefaultGuard {
    fn check(&self, _context: &ContextMap, _history: &[crate::types::VpeEvent]) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "Default"
    }
}

pub fn register_builtins(builder: &mut GuardRegistryBuilder) {
    let factory: GuardFactory = Arc::new(|_params: &Value| -> Result<Box<dyn Guard>, VpeError> {
        Ok(Box::new(DefaultGuard))
    });

    builder.register_guard("Default", factory);
}
