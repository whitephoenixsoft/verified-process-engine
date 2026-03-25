use crate::compiler::source::LawSource;
use crate::error::{CompileError, SchemaError};
use crate::registry::GuardRegistry;
use crate::schema::{validate_schema, DomainSchema};
use std::collections::HashSet;

pub fn validate_law(
    schema: &DomainSchema,
    law: &LawSource,
    registry: &GuardRegistry,
) -> Result<(), CompileError> {
    validate_schema(schema).map_err(map_schema_error)?;

    if law.states.is_empty() {
        return Err(CompileError::InvalidLaw(
            "law must contain at least one state".into(),
        ));
    }

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
            }
        }
    }

    Ok(())
}

fn map_schema_error(err: SchemaError) -> CompileError {
    CompileError::InvalidLaw(err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::GuardRegistryBuilder;
    use crate::schema::{DomainSchema, FieldDefinition, SchemaFieldType};
    use crate::compiler::source::{GuardSource, StateSource, TransitionSource};
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
                            params: BTreeMap::new(),
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
}
