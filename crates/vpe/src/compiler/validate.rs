use crate::compiler::source::{GuardSource, LawSource, EffectMode, StateSource, TransitionSource};
use crate::error::{CompileError, SchemaError};
use crate::registry::GuardRegistry;
use crate::schema::{validate_schema, DomainSchema, SchemaFieldType};
use serde_json::Value;
use std::collections::{HashSet, VecDeque};

pub fn validate_law(
    schema: &DomainSchema,
    law: &LawSource,
    registry: &GuardRegistry,
) -> Result<Vec<String>, CompileError> {
    validate_schema(schema).map_err(map_schema_error)?;
    
    if law.domain != schema.domain {
        return Err(CompileError::InvalidLaw(format!(
            "law domain '{}' does not match schema domain '{}'",
            law.domain, schema.domain
        )));
    }

    if law.schema_version != schema.version {
        return Err(CompileError::InvalidLaw(format!(
            "law schema_version '{}' does not match schema version '{}'",
            law.schema_version, schema.version
        )));
    }
    
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

            validate_transition_effects(law, state, transition)?;
            
            for guard in &transition.guards {
                if registry.get(&guard.guard_type).is_none() {
                    return Err(CompileError::UnknownGuardType(
                        guard.guard_type.clone(),
                    ));
                }

                validate_guard_source(schema, guard)?;
            }
        }

        validate_transient_states(law)?;
        validate_auto_tick_structure(law)?;
        
        if state.transitions.is_empty() {
            warnings.push(format!("state '{}' is terminal", state.name));
        }
    }

    warnings.extend(find_unreachable_states(law));

    Ok(warnings)
}

fn validate_auto_tick_structure(law: &LawSource) -> Result<(), CompileError> {
    for state in &law.states {
        for transition in &state.transitions {
            let is_auto_tick = transition.action == "AUTO_TICK";

            if is_auto_tick && state.is_transient == false {
                // allowed for now; no failure
            }

            if is_auto_tick && transition.priority == 0 {
                // allowed; compiler already sorts deterministically
            }
        }
    }

    validate_auto_tick_cycles(law)
}

fn validate_auto_tick_cycles(law: &LawSource) -> Result<(), CompileError> {
    use std::collections::HashSet;

    fn dfs(
        law: &LawSource,
        state_name: &str,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), CompileError> {
        if visited.contains(state_name) {
            return Ok(());
        }

        if !visiting.insert(state_name.to_string()) {
            return Err(CompileError::InvalidLaw(format!(
                "AUTO_TICK cycle detected involving state '{}'",
                state_name
            )));
        }

        if let Some(state) = law.states.iter().find(|s| s.name == state_name) {
            for transition in &state.transitions {
                if transition.action == "AUTO_TICK" {
                    dfs(law, &transition.to, visiting, visited)?;
                }
            }
        }

        visiting.remove(state_name);
        visited.insert(state_name.to_string());
        Ok(())
    }

    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();

    for state in &law.states {
        dfs(law, &state.name, &mut visiting, &mut visited)?;
    }

    Ok(())
}

fn validate_transient_states(law: &LawSource) -> Result<(), CompileError> {
    for state in &law.states {
        if state.is_transient && state.transitions.is_empty() {
            return Err(CompileError::InvalidLaw(format!(
                "transient state '{}' must define at least one outgoing transition",
                state.name
            )));
        }
    }

    for state in &law.states {
        for transition in &state.transitions {
            let has_tracked_effect = transition.effects.iter().any(|effect| {
                matches!(effect.mode.clone().unwrap_or(EffectMode::Untracked), EffectMode::Tracked)
            });

            if !has_tracked_effect {
                continue;
            }

            let target_state = law
                .states
                .iter()
                .find(|s| s.name == transition.to)
                .ok_or_else(|| CompileError::UnknownTargetState(transition.to.clone()))?;

            if target_state.transitions.is_empty() {
                return Err(CompileError::InvalidLaw(format!(
                    "tracked effect transition targets transient state '{}' but it has no exits",
                    target_state.name
                )));
            }
        }
    }

    Ok(())
}

fn validate_transition_effects(
    law: &LawSource,
    _state: &StateSource,
    transition: &TransitionSource,
) -> Result<(), CompileError> {
    let has_tracked_effect = transition.effects.iter().any(|effect| {
        matches!(effect.mode.clone().unwrap_or(EffectMode::Untracked), EffectMode::Tracked)
    });

    if !has_tracked_effect {
        return Ok(());
    }

    let target_state = law
        .states
        .iter()
        .find(|s| s.name == transition.to)
        .ok_or_else(|| CompileError::UnknownTargetState(transition.to.clone()))?;

    if !target_state.is_transient {
        return Err(CompileError::InvalidLaw(format!(
            "tracked effects require transient target state, but transition to '{}' is not transient",
            transition.to
        )));
    }

    Ok(())
}

fn validate_guard_source(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    match guard.guard_type.as_str() {
        "Default" => Ok(()),
        "GreaterThan" => validate_greater_than_guard(schema, guard),
        "Equals" => validate_equals_guard(schema, guard),
        "OccurredWithin" => validate_occurred_within_guard(guard),
        "TimeElapsed" => validate_time_elapsed_guard(guard),
        "FieldsEqual" => validate_fields_equal_guard(schema, guard),
        "Exists" => validate_exists_guard(schema, guard),
        "MissingField" => validate_missing_field_guard(schema, guard),
        "InSet" => validate_in_set_guard(schema, guard),
        "NotInSet" => validate_in_set_guard(schema, guard),
        _ => Ok(()),
    }
}

fn validate_exists_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let path = guard
        .params
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("Exists requires string field 'path'".into()))?;

    schema
        .resolve_path_type(path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{path}'")))?;

    Ok(())
}

fn validate_missing_field_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let path = guard
        .params
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("MissingField requires string field 'path'".into()))?;

    schema
        .resolve_path_type(path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{path}'")))?;

    Ok(())
}

fn validate_in_set_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let path = guard
        .params
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("InSet/NotInSet requires string field 'path'".into()))?;

    let values = guard
        .params
        .get("values")
        .and_then(Value::as_array)
        .ok_or_else(|| CompileError::InvalidLaw("InSet/NotInSet requires array field 'values'".into()))?;

    let field = schema
        .resolve_field(path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{path}'")))?;

    for value in values {
        if !value_matches_field(field.clone(), value) {
            return Err(CompileError::TypeMismatch(format!(
                "InSet/NotInSet on '{path}' received value incompatible with field type"
            )));
        }
    }

    Ok(())
}

fn validate_fields_equal_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let left_path = guard
        .params
        .get("left_path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("FieldsEqual requires string field 'left_path'".into()))?;

    let right_path = guard
        .params
        .get("right_path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("FieldsEqual requires string field 'right_path'".into()))?;

    let left_type = schema
        .resolve_path_type(left_path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{left_path}'")))?;

    let right_type = schema
        .resolve_path_type(right_path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{right_path}'")))?;

    if field_types_compatible(left_type, right_type) {
        Ok(())
    } else {
        Err(CompileError::TypeMismatch(format!(
            "FieldsEqual requires compatible field types, but '{left_path}' and '{right_path}' differ"
        )))
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

fn validate_equals_guard(
    schema: &DomainSchema,
    guard: &GuardSource,
) -> Result<(), CompileError> {
    let path = guard
        .params
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| CompileError::InvalidLaw("Equals requires string field 'path'".into()))?;

    let value = guard
        .params
        .get("value")
        .ok_or_else(|| CompileError::InvalidLaw("Equals requires field 'value'".into()))?;

    let field = schema
        .resolve_field(path)
        .ok_or_else(|| CompileError::UnresolvedReference(format!("unknown field path '{path}'")))?;

    if value_matches_field(field.clone(), value) {
        Ok(())
    } else {
        Err(CompileError::TypeMismatch(format!(
            "Equals on '{path}' received value incompatible with field type"
        )))
    }
}

fn validate_occurred_within_guard(guard: &GuardSource) -> Result<(), CompileError> {
    let target_action = guard.params.get("target_action").and_then(Value::as_str);
    let window_seconds = guard.params.get("window_seconds").and_then(Value::as_u64);

    if target_action.is_none() {
        return Err(CompileError::InvalidLaw(
            "OccurredWithin requires string field 'target_action'".into(),
        ));
    }

    if window_seconds.is_none() {
        return Err(CompileError::InvalidLaw(
            "OccurredWithin requires numeric field 'window_seconds'".into(),
        ));
    }

    Ok(())
}

fn validate_time_elapsed_guard(guard: &GuardSource) -> Result<(), CompileError> {
    let seconds = guard.params.get("seconds").and_then(Value::as_u64);

    if seconds.is_none() {
        return Err(CompileError::InvalidLaw(
            "TimeElapsed requires numeric field 'seconds'".into(),
        ));
    }

    Ok(())
}

fn value_matches_field(
    field: crate::schema::domain_schema::FieldDefinitionRef<'_>,
    value: &Value,
) -> bool {
    match field.field_type() {
        SchemaFieldType::String => value.is_string(),
        SchemaFieldType::Number => value.is_number(),
        SchemaFieldType::Boolean => value.is_boolean(),
        SchemaFieldType::DateTime => value.is_number(),
        SchemaFieldType::Duration => value.is_number(),
        SchemaFieldType::Enum => {
            let Some(actual) = value.as_str() else {
                return false;
            };

            match field.enum_values() {
                Some(values) => values.iter().any(|v| v == actual),
                None => false,
            }
        }
    }
}

fn field_types_compatible(left: &SchemaFieldType, right: &SchemaFieldType) -> bool {
    match (left, right) {
        (SchemaFieldType::String, SchemaFieldType::String) => true,
        (SchemaFieldType::Number, SchemaFieldType::Number) => true,
        (SchemaFieldType::Boolean, SchemaFieldType::Boolean) => true,
        (SchemaFieldType::DateTime, SchemaFieldType::DateTime) => true,
        (SchemaFieldType::Duration, SchemaFieldType::Duration) => true,
        (SchemaFieldType::Enum, SchemaFieldType::Enum) => true,
        _ => false,
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
            namespaces: crate::schema::SchemaNamespaces {
                rec: vec![
                    FieldDefinition {
                        name: "amount".into(),
                        field_type: SchemaFieldType::Number,
                        description: None,
                        enum_values: None,
                    },
                    FieldDefinition {
                        name: "status".into(),
                        field_type: SchemaFieldType::String,
                        description: None,
                        enum_values: None,
                    },
                ],
                ext: vec![
                    FieldDefinition {
                        name: "status".into(),
                        field_type: SchemaFieldType::String,
                        description: None,
                        enum_values: None,
                    },
                ],
                calc: vec![],
            },
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
            schema_version: "1.0.0".into(),
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

    fn valid_law_with_status_comparison() -> LawSource {
        LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
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

    #[test]
    fn rejects_equals_with_missing_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Equals".into(),
            params: BTreeMap::from([("value".into(), json!(100))]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_equals_with_unknown_field() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Equals".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.unknown")),
                ("value".into(), json!(100)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn rejects_equals_with_wrong_type() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Equals".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.amount")),
                ("value".into(), json!("wrong")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::TypeMismatch(_))));
    }

    #[test]
    fn validates_equals_with_matching_type() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Equals".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.amount")),
                ("value".into(), json!(100)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_occurred_within_without_target_action() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "OccurredWithin".into(),
            params: BTreeMap::from([
                ("window_seconds".into(), json!(3600)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_occurred_within_without_window_seconds() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "OccurredWithin".into(),
            params: BTreeMap::from([
                ("target_action".into(), json!("FraudCheck")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn validates_occurred_within() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "OccurredWithin".into(),
            params: BTreeMap::from([
                ("target_action".into(), json!("FraudCheck")),
                ("window_seconds".into(), json!(3600)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_time_elapsed_without_seconds() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "TimeElapsed".into(),
            params: BTreeMap::new(),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn validates_time_elapsed() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "TimeElapsed".into(),
            params: BTreeMap::from([
                ("seconds".into(), json!(300)),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_fields_equal_without_left_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("right_path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_fields_equal_without_right_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("left_path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_fields_equal_with_unknown_left_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("left_path".into(), json!("rec.unknown")),
                ("right_path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn rejects_fields_equal_with_unknown_right_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("left_path".into(), json!("rec.status")),
                ("right_path".into(), json!("rec.unknown")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn rejects_fields_equal_with_incompatible_types() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("left_path".into(), json!("rec.amount")),
                ("right_path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::TypeMismatch(_))));
    }

    #[test]
    fn validates_fields_equal_with_compatible_types() {
        let mut law = valid_law_with_status_comparison();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "FieldsEqual".into(),
            params: BTreeMap::from([
                ("left_path".into(), json!("rec.status")),
                ("right_path".into(), json!("ext.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_exists_without_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Exists".into(),
            params: BTreeMap::new(),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_exists_with_unknown_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Exists".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.unknown")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn validates_exists_with_known_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "Exists".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_missing_field_without_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "MissingField".into(),
            params: BTreeMap::new(),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_missing_field_with_unknown_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "MissingField".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.unknown")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::UnresolvedReference(_))));
    }

    #[test]
    fn validates_missing_field_with_known_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "MissingField".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_in_set_without_path() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "InSet".into(),
            params: BTreeMap::from([
                ("values".into(), json!(["A", "B"])),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_in_set_without_values() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "InSet".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_in_set_with_non_array_values() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "InSet".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
                ("values".into(), json!("not-an-array")),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_in_set_with_type_mismatch_values() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "InSet".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
                ("values".into(), json!(["OK", 123])),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::TypeMismatch(_))));
    }

    #[test]
    fn validates_in_set() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "InSet".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
                ("values".into(), json!(["Draft", "Review"])),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn validates_not_in_set() {
        let mut law = valid_law();
        law.states[0].transitions[0].guards[0] = GuardSource {
            guard_type: "NotInSet".into(),
            params: BTreeMap::from([
                ("path".into(), json!("rec.status")),
                ("values".into(), json!(["Draft", "Review"])),
            ]),
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn allows_untracked_effect_into_stable_state() {
        let mut law = valid_law();
        law.states[0].transitions[0].effects = vec![
            crate::compiler::source::EffectSource {
                effect_type: "SendEmail".into(),
                target: Some("Notification".into()),
                action: Some("Send".into()),
                params: None,
                mode: Some(crate::compiler::source::EffectMode::Untracked),
            }
        ];

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_tracked_effect_into_stable_state() {
        let mut law = valid_law();
        law.states[0].transitions[0].effects = vec![
            crate::compiler::source::EffectSource {
                effect_type: "ChargeCard".into(),
                target: Some("Payments".into()),
                action: Some("Capture".into()),
                params: None,
                mode: Some(crate::compiler::source::EffectMode::Tracked),
            }
        ];

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_transient_state_with_no_outgoing_transitions() {
        let law = LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![TransitionSource {
                        action: "Submit".into(),
                        to: "PendingPayment".into(),
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
                    name: "PendingPayment".into(),
                    is_transient: true,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_tracked_effect_targeting_transient_state_with_no_exits() {
        let law = LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![TransitionSource {
                        action: "Submit".into(),
                        to: "PendingPayment".into(),
                        priority: 0,
                        guards: vec![GuardSource {
                            guard_type: "Default".into(),
                            params: BTreeMap::<String, Value>::new(),
                        }],
                        effects: vec![
                            crate::compiler::source::EffectSource {
                                effect_type: "ChargeCard".into(),
                                target: Some("Payments".into()),
                                action: Some("Capture".into()),
                                params: None,
                                mode: Some(crate::compiler::source::EffectMode::Tracked),
                            }
                        ],
                        comment: None,
                    }],
                },
                StateSource {
                    name: "PendingPayment".into(),
                    is_transient: true,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn allows_tracked_effect_targeting_transient_state_with_exit() {
        let law = LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![TransitionSource {
                        action: "Submit".into(),
                        to: "PendingPayment".into(),
                        priority: 0,
                        guards: vec![GuardSource {
                            guard_type: "Default".into(),
                            params: BTreeMap::<String, Value>::new(),
                        }],
                        effects: vec![
                            crate::compiler::source::EffectSource {
                                effect_type: "ChargeCard".into(),
                                target: Some("Payments".into()),
                                action: Some("Capture".into()),
                                params: None,
                                mode: Some(crate::compiler::source::EffectMode::Tracked),
                            }
                        ],
                        comment: None,
                    }],
                },
                StateSource {
                    name: "PendingPayment".into(),
                    is_transient: true,
                    transitions: vec![TransitionSource {
                        action: "PaymentSucceeded".into(),
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
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_auto_tick_cycle() {
        let law = LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "A".into(),
            states: vec![
                StateSource {
                    name: "A".into(),
                    is_transient: true,
                    transitions: vec![TransitionSource {
                        action: "AUTO_TICK".into(),
                        to: "B".into(),
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
                    name: "B".into(),
                    is_transient: true,
                    transitions: vec![TransitionSource {
                        action: "AUTO_TICK".into(),
                        to: "A".into(),
                        priority: 0,
                        guards: vec![GuardSource {
                            guard_type: "Default".into(),
                            params: BTreeMap::<String, Value>::new(),
                        }],
                        effects: vec![],
                        comment: None,
                    }],
                },
            ],
            migration_rules: vec![],
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn allows_acyclic_auto_tick_chain() {
        let law = LawSource {
            domain: "TestDomain".into(),
            schema_version: "1.0.0".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "A".into(),
            states: vec![
                StateSource {
                    name: "A".into(),
                    is_transient: true,
                    transitions: vec![TransitionSource {
                        action: "AUTO_TICK".into(),
                        to: "B".into(),
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
                    name: "B".into(),
                    is_transient: true,
                    transitions: vec![TransitionSource {
                        action: "AUTO_TICK".into(),
                        to: "C".into(),
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
                    name: "C".into(),
                    is_transient: false,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        };

        let result = validate_law(&schema(), &law, &registry());
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_law_with_mismatched_domain() {
        let mut law = valid_law();
        law.domain = "OtherDomain".into();

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn rejects_law_with_mismatched_schema_version() {
        let mut law = valid_law();
        law.schema_version = "2.0.0".into();

        let result = validate_law(&schema(), &law, &registry());
        assert!(matches!(result, Err(CompileError::InvalidLaw(_))));
    }

    #[test]
    fn resolves_builtin_system_schema_fields() {
        let schema = schema();
        assert!(matches!(
            schema.resolve_path_type("sys.now"),
            Some(crate::schema::SchemaFieldType::DateTime)
        ));
        assert!(matches!(
            schema.resolve_path_type("sys.trace_id"),
            Some(crate::schema::SchemaFieldType::String)
        ));
    }
}
