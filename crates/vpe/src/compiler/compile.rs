
use crate::compiler::compiled::{CompiledProcess, Edge, Node};
use crate::compiler::digest::compute_digest;
use crate::compiler::manifest::build_manifests;
use crate::compiler::source::{EffectSource, LawSource};
use crate::compiler::validate::validate_law;
use crate::error::{CompileError, VpeError};
use crate::registry::GuardRegistry;
use crate::schema::DomainSchema;
use crate::types::{ProcessRef, StateManifest, VpeEffect};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct VpeCompiler {
    registry: GuardRegistry,
}

pub struct CompilationResult {
    pub process: CompiledProcess,
    pub report: RegistrationReport,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub process: ProcessRef,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RegistrationReport {
    pub process: ProcessRef,
    pub digest: String,
    pub manifests: HashMap<String, StateManifest>,
    pub warnings: Vec<String>,
}

impl VpeCompiler {
    pub fn with_registry(registry: GuardRegistry) -> Self {
        Self { registry }
    }

    pub fn validate(
        &self,
        schema: &DomainSchema,
        law: &LawSource,
    ) -> Result<ValidationReport, VpeError> {
        let warnings = validate_law(schema, law, &self.registry)?;
        let process = ProcessRef::new(&law.domain, &law.process, &law.version);

        Ok(ValidationReport { process, warnings })
    }

    pub fn compile(
        &self,
        schema: &DomainSchema,
        law: &LawSource,
    ) -> Result<CompilationResult, VpeError> {
        let validation = self.validate(schema, law)?;

        let process_ref = ProcessRef::new(&law.domain, &law.process, &law.version);
        let manifests = build_manifests(law);
        let digest = compute_digest(law)?;

        let state_to_idx = build_state_index(law);
        let initial_state_idx = *state_to_idx
            .get(&law.initial_state)
            .ok_or_else(|| VpeError::Compile(CompileError::InitialStateNotFound(law.initial_state.clone())))?;

        let mut nodes = Vec::with_capacity(law.states.len());

        for state in &law.states {
            let mut transitions = Vec::with_capacity(state.transitions.len());

            for transition in &state.transitions {
                let target_idx = *state_to_idx
                    .get(&transition.to)
                    .ok_or_else(|| VpeError::Compile(CompileError::UnknownTargetState(transition.to.clone())))?;

                let mut guards = Vec::with_capacity(transition.guards.len());
                for guard_source in &transition.guards {
                    let factory = self.registry.get(&guard_source.guard_type).ok_or_else(|| {
                        VpeError::Compile(CompileError::UnknownGuardType(guard_source.guard_type.clone()))
                    })?;

                    let params = guard_params_to_value(&guard_source.params);
                    let guard = factory(&params)?;
                    guards.push(guard);
                }

                let effects = transition
                    .effects
                    .iter()
                    .map(compile_effect)
                    .collect::<Vec<_>>();

                transitions.push(Edge {
                    action: transition.action.clone(),
                    target_idx,
                    priority: transition.priority,
                    guards,
                    effects,
                });
            }

            transitions.sort_by(|a, b| b.priority.cmp(&a.priority));

            nodes.push(Node {
                name: state.name.clone(),
                transitions,
            });
        }

        let process = CompiledProcess::new(
            process_ref.clone(),
            digest.clone(),
            manifests.clone(),
            initial_state_idx,
            nodes,
        );

        let report = RegistrationReport {
            process: process_ref,
            digest,
            manifests,
            warnings: validation.warnings,
        };

        Ok(CompilationResult { process, report })
    }
}

fn build_state_index(law: &LawSource) -> HashMap<String, usize> {
    law.states
        .iter()
        .enumerate()
        .map(|(idx, state)| (state.name.clone(), idx))
        .collect()
}

fn guard_params_to_value(params: &std::collections::BTreeMap<String, Value>) -> Value {
    let map: Map<String, Value> = params.clone().into_iter().collect();
    Value::Object(map)
}

fn compile_effect(effect: &EffectSource) -> VpeEffect {
    let params = match &effect.params {
        Some(Value::Object(map)) => map.clone(),
        Some(_) | None => Map::new(),
    };

    VpeEffect {
        effect_type: effect.effect_type.clone(),
        target: effect.target.clone(),
        action: effect.action.clone(),
        params,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::source::{EffectSource, GuardSource, StateSource, TransitionSource};
    use crate::registry::GuardRegistryBuilder;
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

    fn compiler() -> VpeCompiler {
        let registry = GuardRegistryBuilder::new()
            .with_builtins()
            .build()
            .expect("registry should build");
        VpeCompiler::with_registry(registry)
    }

    fn law_with_priorities() -> LawSource {
        LawSource {
            domain: "TestDomain".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![
                        TransitionSource {
                            action: "Submit".into(),
                            to: "Approved".into(),
                            priority: 1,
                            guards: vec![GuardSource {
                                guard_type: "Default".into(),
                                params: BTreeMap::<String, Value>::new(),
                            }],
                            effects: vec![],
                            comment: None,
                        },
                        TransitionSource {
                            action: "Submit".into(),
                            to: "Approved".into(),
                            priority: 10,
                            guards: vec![GuardSource {
                                guard_type: "GreaterThan".into(),
                                params: BTreeMap::from([
                                    ("path".into(), json!("rec.amount")),
                                    ("value".into(), json!(100)),
                                ]),
                            }],
                            effects: vec![EffectSource {
                                effect_type: "Notify".into(),
                                target: Some("Email".into()),
                                action: Some("Send".into()),
                                params: Some(json!({ "template": "approved" })),
                            }],
                            comment: None,
                        },
                    ],
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
    fn compile_resolves_initial_state_index() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        assert_eq!(result.process.initial_state_idx(), 0);
    }

    #[test]
    fn compile_builds_nodes_for_states() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        assert_eq!(result.process.nodes().len(), 2);
        assert_eq!(result.process.nodes()[0].name, "Draft");
        assert_eq!(result.process.nodes()[1].name, "Approved");
    }

    #[test]
    fn compile_resolves_target_indices() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        let draft = &result.process.nodes()[0];
        assert!(draft.transitions.iter().all(|t| t.target_idx == 1));
    }

    #[test]
    fn compile_sorts_transitions_by_priority_descending() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        let draft = &result.process.nodes()[0];
        assert_eq!(draft.transitions.len(), 2);
        assert_eq!(draft.transitions[0].priority, 10);
        assert_eq!(draft.transitions[1].priority, 1);
    }

    #[test]
    fn compile_hydrates_guards() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        let draft = &result.process.nodes()[0];
        assert_eq!(draft.transitions[0].guards.len(), 1);
        assert_eq!(draft.transitions[0].guards[0].name(), "GreaterThan");
        assert_eq!(draft.transitions[1].guards[0].name(), "Default");
    }

    #[test]
    fn compile_maps_effects() {
        let result = compiler().compile(&schema(), &law_with_priorities()).unwrap();
        let draft = &result.process.nodes()[0];
        let effect = &draft.transitions[0].effects[0];

        assert_eq!(effect.effect_type, "Notify");
        assert_eq!(effect.target.as_deref(), Some("Email"));
        assert_eq!(effect.action.as_deref(), Some("Send"));
        assert_eq!(effect.params.get("template").and_then(|v| v.as_str()), Some("approved"));
    }
}