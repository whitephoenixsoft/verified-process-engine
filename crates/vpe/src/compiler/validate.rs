use serde_json::Value;
use crate::compiler::source::{GuardSource, LawSource};
use crate::error::{CompileError, SchemaError};
use crate::registry::GuardRegistry;
use crate::schema::{validate_schema, DomainSchema, SchemaFieldType};
use std::collections::{HashSet, VecDeque};

pub fn validate_law(
    schema: &DomainSchema,
    law: &LawSource,
    registry: &GuardRegistry,
) -> Result<Vec<String>, CompileError> {
    validate_schema(schema).map_err(map_schema_error)?;

    if law.states.is_empty() {
        return Err(CompileError::InvalidLaw(
            "law must contain at least one state".into(),
        ));
    }

    let mut warnings = Vec::new();
    let mut state_names = HashSet::new();

    for state in &law.states {
        if !state_names.insert(state.name.clone()) {
            return Err(CompileError::DuplicateState(state.name.clone()));
        }
    }

    if !state_names.contains(&law.initial_state) {
        return Err(CompileError::InitialStateNotFound(
            law.initial_state.clone(),
        ));
    }

    for state in &law.states {
        for transition in &state.transitions {
            if !state_names.contains(&transition.to) {
                return Err(CompileError::UnknownTargetState(
                    transition.to.clone(),
                ));
            }

            for guard in &transition.guards {
                if registry.get(&guard.guard_type).is_none() {
                    return Err(CompileError::UnknownGuardType(
                        guard.guard_type.clone(),
                    ));
                }

                validate_guard_source(schema, guard)?;
            }
        }

        if state.transitions.is_empty() {
            warnings.push(format!("state '{}' is terminal", state.name));
        }
    }

    warnings.extend(find_unreachable_states(law));

    Ok(warnings)
}

fn validate_guard_source(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    match guard.guard_type.as_str() {
        "Default" => Ok(()),
        "GreaterThan" => validate_greater_than_guard(schema, guard),
        _ => Ok(()),
    }
}

fn validate_greater_than_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let path = guard
        .params
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("GreaterThan requires string field 'path'".into()))?;

    let value = guard
        .params
        .get("value")
        .ok_or_else(|| CompileError::InvalidLaw("GreaterThan requires field 'value'".into()))?;

    let field_type = schema
        .resolve_path_type(path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{path}'")))?;

    match field_type {
        SchemaFieldType::Number => {
            if value.is_number() {
                Ok(())
            } else {
                Err(CompileError::TypeMismatch(format!(
                    "GreaterThan on '{path}' requires numeric value"
                )))
            }
        }
        _ => Err(CompileError::TypeMismatch(format!(
            "GreaterThan requires a numeric field, but '{path}' is not numeric"
        ))),
    }
}

fn find_unreachable_states(law: &LawSource) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(law.initial_state.clone());

    while let Some(state_name) = queue.pop_front() {
        if !visited.insert(state_name.clone()) {
            continue;
        }

        if let Some(state) = law.states.iter().find(|s| s.name == state_name) {
            for transition in &state.transitions {
                queue.push_back(transition.to.clone());
            }
        }
    }

    for state in &law.states {
        if !visited.contains(&state.name) {
            warnings.push(format!("state '{}' is unreachable", state.name));
        }
    }

    warnings
}

fn map_schema_error(err: SchemaError) -> CompileError {
    CompileError::InvalidLaw(err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::source::{GuardSource, StateSource, TransitionSource};
    use crate::registry::{GuardRegistry, GuardRegistryBuilder};
    use crate::schema::{DomainSchema, FieldDefinition, SchemaFieldType};
    use serde_json::{json, Value};
    use std::collections::BTreeMap;

    fn schema() -> DomainSchema {
        DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            fields: vec![FieldDefinition {
                name: "amount".into(),
                field_type: SchemaFieldType::Number,
                description: None,
            }],
        }
    }

    fn registry() -> GuardRegistry {
        GuardRegistryBuilder::new()
            .with_builtins()
            .build()
            .expect("registry should build")
    }

    fn valid_law() -> LawSource {
        LawSource {
            domain: "TestDomain".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![TransitionSource {
                        action: "Submit".into(),
                        to: "Approved".into(),
                        priority: 0,
                        guards: vec![GuardSource {
                            guard_type: "Default".into(),
                            params: BTreeMap::<String, Value>::new(),
                        }],
                        effects: vec![],
                        comment: None,
                    }],
                },
                StateSource {
                    name: "Approved".into(),
                    is_transient: false,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        }
    }

    #[test]
    fn validates_a_basic_law() {
        let result = validate_law(&schema(), &valid_law(), &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_duplicate_states() {
        let mut law = valid_law();
        law.states.push(StateSource {
            name: "Draft".into(),
            is_transient: false,
            transitions: vec![],
        });

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::DuplicateState(_))));
    }

    #[test]
    fn rejects_missing_initial_state() {
        let mut law = valid_law();
        law.initial_state = "Missing".into();

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InitialStateNotFound(_))));
    }

    #[test]
    fn rejects_unknown_target_state() {
        let mut law = valid_law();
        law.states[0].transitions[0].to = "Missing".into();

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnknownTargetState(_))));
    }

    #[test]
    fn rejects_unknown_guard_type() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0].guard_type = "NotRegistered".into();

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnknownGuardType(_))));
    }

    #[test]
    fn warns_on_terminal_states() {
        let result = validate_law(&schema(), &valid_law(), &registry()).unwrap();
        assert!(result.iter().any(|w| w.contains("terminal")));
    }

    #[test]
    fn warns_on_unreachable_states() {
        let mut law = valid_law();
        law.states.push(StateSource {
            name: "Orphan".into(),
            is_transient: false,
            transitions: vec![],
        });

        let result = validate_law(&schema(), &law, &registry()).unwrap();
        assert!(result.iter().any(|w| w.contains("unreachable")));
    }

    #[test]
    fn rejects_greater_than_with_missing_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "GreaterThan".into(),
            params: BTreeMap::from([("value".into(), json!(100))]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_greater_than_with_unknown_field() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "GreaterThan".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.missing_field")),
                ("value".into(), json!(100)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn rejects_greater_than_with_wrong_value_type() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "GreaterThan".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.amount")),
                ("value".into(), json!("not-a-number")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::TypeMismatch(_))));
    }

    #[test]
    fn validates_greater_than_with_number_field() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "GreaterThan".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.amount")),
                ("value".into(), json!(100)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }
}
