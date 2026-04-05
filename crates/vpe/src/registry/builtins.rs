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

#[derive(Debug)]
struct FieldsEqualGuard {
    left_path: String,
    right_path: String,
}

impl Guard for FieldsEqualGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        match (context.get(&self.left_path), context.get(&self.right_path)) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![
                ContextRequirement::Field(self.left_path.clone()),
                ContextRequirement::Field(self.right_path.clone()),
            ],
        }
    }

    fn name(&self) -> &'static str {
        "FieldsEqual"
    }
}

#[derive(Debug)]
struct ExistsGuard {
    path: String,
}

impl Guard for ExistsGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context.get(&self.path).map(|v| !v.is_null()).unwrap_or(false)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "Exists"
    }
}

#[derive(Debug)]
struct MissingFieldGuard {
    path: String,
}

impl Guard for MissingFieldGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context.get(&self.path).map(|v| v.is_null()).unwrap_or(true)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "MissingField"
    }
}

#[derive(Debug)]
struct InSetGuard {
    path: String,
    values: Vec<Value>,
}

impl Guard for InSetGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context
            .get(&self.path)
            .map(|actual| self.values.iter().any(|candidate| candidate == actual))
            .unwrap_or(false)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "InSet"
    }
}

#[derive(Debug)]
struct NotInSetGuard {
    path: String,
    values: Vec<Value>,
}

impl Guard for NotInSetGuard {
    fn check(&self, context: &ContextMap, _history: &[VpeEvent]) -> bool {
        context
            .get(&self.path)
            .map(|actual| self.values.iter().all(|candidate| candidate != actual))
            .unwrap_or(true)
    }

    fn requirements(&self) -> GuardRequirements {
        GuardRequirements {
            history: vec![],
            context: vec![ContextRequirement::Field(self.path.clone())],
        }
    }

    fn name(&self) -> &'static str {
        "NotInSet"
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

    let fields_equal_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let left_path = params
                .get("left_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    VpeError::Unsupported("FieldsEqual requires string field 'left_path'".into())
                })?
                .to_string();

            let right_path = params
                .get("right_path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    VpeError::Unsupported("FieldsEqual requires string field 'right_path'".into())
                })?
                .to_string();

            Ok(Box::new(FieldsEqualGuard {
                left_path,
                right_path,
            }))
        });
    builder.register_guard("FieldsEqual", fields_equal_factory);

    let exists_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| VpeError::Unsupported("Exists requires string field 'path'".into()))?
                .to_string();

            Ok(Box::new(ExistsGuard { path }))
        });
    builder.register_guard("Exists", exists_factory);

    let missing_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| VpeError::Unsupported("MissingField requires string field 'path'".into()))?
                .to_string();

            Ok(Box::new(MissingFieldGuard { path }))
        });
    builder.register_guard("MissingField", missing_factory);

    let in_set_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| VpeError::Unsupported("InSet requires string field 'path'".into()))?
                .to_string();

            let values = params
                .get("values")
                .and_then(|v| v.as_array())
                .ok_or_else(|| VpeError::Unsupported("InSet requires array field 'values'".into()))?
                .clone();

            Ok(Box::new(InSetGuard { path, values }))
        });
    builder.register_guard("InSet", in_set_factory);

    let not_in_set_factory: GuardFactory =
        Arc::new(|params: &Value| -> Result<Box<dyn Guard>, VpeError> {
            let path = params
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| VpeError::Unsupported("NotInSet requires string field 'path'".into()))?
                .to_string();

            let values = params
                .get("values")
                .and_then(|v| v.as_array())
                .ok_or_else(|| VpeError::Unsupported("NotInSet requires array field 'values'".into()))?
                .clone();

            Ok(Box::new(NotInSetGuard { path, values }))
        });
    builder.register_guard("NotInSet", not_in_set_factory);
}
