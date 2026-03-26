use crate::error::VpeError;
use crate::registry::guard_registry::{GuardFactory, GuardRegistryBuilder};
use crate::registry::Guard;
use crate::types::{ContextMap, ContextRequirement, GuardRequirements, VpeEvent};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug)]
struct DefaultGuard;

impl Guard for DefaultGuard {
    fn check(&self, _context: &ContextMap, _history: &[VpeEvent]) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "Default"
    }
}

#[derive(Debug)]
struct GreaterThanGuard {
    path: String,
    value: f64,
}

impl Guard for GreaterThanGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context
            .get(&self.path)
            .and_then(|v| v.as_f64())
            .map(|actual| actual > self.value)
            .unwrap_or(false)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "GreaterThan"
    }
}

pub fn register_builtins(builder: &mut GuardRegistryBuilder) {
    let default_factory: GuardFactory =
        Arc::new(|_params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            Ok(Box::new(DefaultGuard))
        });

    builder.register_guard("Default", default_factory);

    let gt_factory: GuardFactory = Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
        let path = params
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VpeError::Unsupported("GreaterThan requires string field 'path'".into()))?
            .to_string();

        let value = params
            .get("value")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| VpeError::Unsupported("GreaterThan requires numeric field 'value'".into()))?;

        Ok(Box::new(GreaterThanGuard { path, value }))
    });

    builder.register_guard("GreaterThan", gt_factory);
}
