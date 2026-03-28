
use crate::compiler::compiled::{CompiledProcess, Edge};
use crate::error::{RuntimeError, VpeError};
use crate::types::{PlannedEvent, VpeEventKind, VpeRequest, VpeVerdict};
use serde_json::json;

pub fn evaluate(
    process: &CompiledProcess,
    request: &VpeRequest,
) -> Result<VpeVerdict, VpeError> {
    let anchor = &request.chronicle.anchor;

    if anchor.state_after != request.current_state {
        return Err(VpeError::Runtime(RuntimeError::Desync {
            expected: anchor.state_after.clone(),
            provided: request.current_state.clone(),
        }));
    }

    let node = process
        .nodes()
        .iter()
        .find(|n| n.name == request.current_state)
        .ok_or_else(|| VpeError::Runtime(RuntimeError::UnknownState(request.current_state.clone())))?;

    let candidates: Vec<&Edge> = node
        .transitions
        .iter()
        .filter(|t| t.action == request.action)
        .collect();

    if candidates.is_empty() {
        return Err(VpeError::Runtime(RuntimeError::NoTransitionFound {
            state: request.current_state.clone(),
            action: request.action.clone(),
        }));
    }

    for transition in candidates {
        let passed = transition
            .guards
            .iter()
            .all(|guard| guard.check(&request.context, &request.chronicle.events));

        if passed {
            let next_state = process.nodes()[transition.target_idx].name.clone();

            return Ok(VpeVerdict {
                process: request.process.clone(),
                trace_id: request.trace_id.clone(),
                previous_state: request.current_state.clone(),
                next_state: next_state.clone(),
                state_patch: Default::default(),
                effects: transition.effects.clone(),
                emitted_events: vec![PlannedEvent {
                    event_kind: VpeEventKind::StateTransition,
                    action: request.action.clone(),
                    state_before: request.current_state.clone(),
                    state_after: next_state,
                    metadata: json!({}),
                }],
            });
        }
    }

    Err(VpeError::Runtime(RuntimeError::NoTransitionFound {
        state: request.current_state.clone(),
        action: request.action.clone(),
    }))
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use crate::compiler::source::{GuardSource, LawSource, StateSource, TransitionSource};
    use crate::compiler::VpeCompiler;
    use crate::error::{RuntimeError, VpeError};
    use crate::registry::GuardRegistryBuilder;
    use crate::schema::{DomainSchema, FieldDefinition, SchemaFieldType};
    use crate::types::{
        ChronicleView, ContextMap, ProcessRef, VpeEvent, VpeEventKind, VpeRequest,
    };
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

    fn law() -> LawSource {
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
                            priority: 10,
                            guards: vec![GuardSource {
                                guard_type: "GreaterThan".into(),
                                params: BTreeMap::from([
                                    ("path".into(), json!("rec.amount")),
                                    ("value".into(), json!(100)),
                                ]),
                            }],
                            effects: vec![],
                            comment: None,
                        },
                        TransitionSource {
                            action: "Submit".into(),
                            to: "Review".into(),
                            priority: 1,
                            guards: vec![GuardSource {
                                guard_type: "Default".into(),
                                params: BTreeMap::<String, Value>::new(),
                            }],
                            effects: vec![],
                            comment: None,
                        },
                    ],
                },
                StateSource {
                    name: "Approved".into(),
                    is_transient: false,
                    transitions: vec![],
                },
                StateSource {
                    name: "Review".into(),
                    is_transient: false,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        }
    }

    fn anchor(state_after: &str) -> VpeEvent {
        VpeEvent {
            event_id: "evt-1".into(),
            parent_event_id: None,
            trace_id: "trace-1".into(),
            timestamp: 1_700_000_000,
            event_kind: VpeEventKind::StateTransition,
            action: "Create".into(),
            state_before: "None".into(),
            state_after: state_after.into(),
            metadata: json!({}),
        }
    }

    fn request(current_state: &str, action: &str, amount: f64) -> VpeRequest {
        let mut context = ContextMap::new();
        context.insert("rec.amount".into(), json!(amount));
        context.insert("sys.now".into(), json!(1_700_000_100_i64));

        let anchor_event = anchor(current_state);

        VpeRequest {
            process: ProcessRef::new("TestDomain", "TestProcess", "1.0.0"),
            trace_id: "trace-1".into(),
            now: 1_700_000_100,
            current_state: current_state.into(),
            action: action.into(),
            context,
            chronicle: ChronicleView {
                anchor: anchor_event.clone(),
                events: vec![anchor_event],
            },
        }
    }

    #[test]
    fn evaluate_takes_highest_priority_matching_transition() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
        let req = request("Draft", "Submit", 150.0);

        let verdict = evaluate(&compiled, &req).unwrap();

        assert_eq!(verdict.previous_state, "Draft");
        assert_eq!(verdict.next_state, "Approved");
        assert_eq!(verdict.emitted_events.len(), 1);
        assert_eq!(verdict.emitted_events[0].state_after, "Approved");
    }

    #[test]
    fn evaluate_falls_back_to_lower_priority_transition_when_guard_fails() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
        let req = request("Draft", "Submit", 50.0);

        let verdict = evaluate(&compiled, &req).unwrap();

        assert_eq!(verdict.next_state, "Review");
    }

    #[test]
    fn evaluate_returns_desync_when_anchor_and_current_state_do_not_match() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
        let mut req = request("Draft", "Submit", 150.0);
        req.chronicle.anchor.state_after = "Approved".into();

        let err = evaluate(&compiled, &req).unwrap_err();

        assert!(matches!(
            err,
            VpeError::Runtime(RuntimeError::Desync { .. })
        ));
    }

    #[test]
    fn evaluate_returns_unknown_state_for_missing_current_state() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
        let req = request("MissingState", "Submit", 150.0);

        let err = evaluate(&compiled, &req).unwrap_err();

        assert!(matches!(
            err,
            VpeError::Runtime(RuntimeError::UnknownState(_))
        ));
    }

    #[test]
    fn evaluate_returns_no_transition_found_for_missing_action() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
        let req = request("Draft", "Cancel", 150.0);

        let err = evaluate(&compiled, &req).unwrap_err();

        assert!(matches!(
            err,
            VpeError::Runtime(RuntimeError::NoTransitionFound { .. })
        ));
    }
}