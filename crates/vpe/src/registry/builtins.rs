use crate::error::VpeError;
use crate::registry::guard_registry::{GuardFactory, GuardRegistryBuilder};
use crate::registry::Guard;
use crate::types::{
    ContextMap, ContextRequirement, GuardRequirements, HistoryRequirement, VpeEvent,
};
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

#[derive(Debug)]
struct EqualsGuard {
    path: String,
    value: Value,
}

impl Guard for EqualsGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context.get(&self.path).map(|v| v == &self.value).unwrap_or(false)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "Equals"
    }
}

#[derive(Debug)]
struct OccurredWithinGuard {
    target_action: String,
    window_seconds: u64,
}

impl Guard for OccurredWithinGuard {
    fn check(&self, _context: &ContextMap, history: &[VpeEvent]) -> bool {
        let now = match history.last() {
            Some(event) => event.timestamp,
            None => return false,
        };

        history.iter().rev().any(|event| {
            event.action == self.target_action
                && (now - event.timestamp) <= self.window_seconds as i64
        })
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![HistoryRequirement::EventsInWindow {
                action: self.target_action.clone(),
                duration_seconds: self.window_seconds,
            }],
            context: vec![],
        }
    }

    fn name(&self) -> &'static str {
        "OccurredWithin"
    }
}

#[derive(Debug)]
struct TimeElapsedGuard {
    seconds: u64,
}

impl Guard for TimeElapsedGuard {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool {
        let now = context.get("sys.now").and_then(|v| v.as_i64()).unwrap_or(0);
        let last = history.last().map(|e| e.timestamp).unwrap_or(0);
        (now - last) >= self.seconds as i64
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![HistoryRequirement::LastTransition],
            context: vec![ContextRequirement::SystemField("sys.now".into())],
        }
    }

    fn name(&self) -> &'static str {
        "TimeElapsed"
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

    let equals_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| VpeError::Unsupported("Equals requires string field 'path'".into()))?
                .to_string();

            let value = params
                .get("value")
                .cloned()
                .ok_or_else(|| VpeError::Unsupported("Equals requires field 'value'".into()))?;

            Ok(Box::new(EqualsGuard { path, value }))
        });
    builder.register_guard("Equals", equals_factory);

    let occurred_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let target_action = params
                .get("target_action")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    VpeError::Unsupported(
                        "OccurredWithin requires string field 'target_action'".into(),
                    )
                })?
                .to_string();

            let window_seconds = params
                .get("window_seconds")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    VpeError::Unsupported(
                        "OccurredWithin requires numeric field 'window_seconds'".into(),
                    )
                })?;

            Ok(Box::new(OccurredWithinGuard {
                target_action,
                window_seconds,
            }))
        });
    builder.register_guard("OccurredWithin", occurred_factory);

    let elapsed_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let seconds = params
                .get("seconds")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| VpeError::Unsupported("TimeElapsed requires field 'seconds'".into()))?;

            Ok(Box::new(TimeElapsedGuard { seconds }))
        });
    builder.register_guard("TimeElapsed", elapsed_factory);
}