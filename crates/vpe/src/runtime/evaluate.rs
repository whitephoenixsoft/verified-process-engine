
use crate::compiler::compiled::{CompiledProcess, Edge};
use crate::error::{RuntimeError, VpeError};
use crate::types::{PlannedEvent, VpeEventKind, VpeRequest, VpeVerdict};
use serde_json::json;

pub fn evaluate(
    process: &CompiledProcess,
    request: &VpeRequest,
) -> Result<VpeVerdict, VpeError> {
    const MAX_AUTO_DEPTH: usize = 32;

    let anchor = &request.chronicle.anchor;

    if anchor.state_after != request.current_state {
        return Err(VpeError::Runtime(RuntimeError::Desync {
            expected: anchor.state_after.clone(),
            provided: request.current_state.clone(),
        }));
    }

    let mut current_state = request.current_state.clone();
    let mut current_action = request.action.clone();
    let mut auto_depth = 0usize;
    let mut accumulated_effects = Vec::new();
    let mut emitted_events = Vec::new();
    let mut previous_state_for_verdict = request.current_state.clone();

    loop {
        let state_idx = process
            .state_index(&current_state)
            .ok_or_else(|| VpeError::Runtime(RuntimeError::UnknownState(current_state.clone())))?;

        let node = &process.nodes()[state_idx];

        let manifest = process
            .manifest(&node.name)
            .ok_or_else(|| VpeError::Runtime(RuntimeError::UnknownState(node.name.clone())))?;

        for required in &manifest.context_requirements {
            match required {
                crate::types::ContextRequirement::Field(field)
                | crate::types::ContextRequirement::SystemField(field) => {
                    if !request.context.contains_key(field) {
                        return Err(VpeError::Runtime(RuntimeError::MissingContextField {
                            field: field.clone(),
                        }));
                    }
                }
            }
        }

        let candidates: Vec<&Edge> = node
            .transitions
            .iter()
            .filter(|t| t.action == current_action)
            .collect();

        if candidates.is_empty() {
            return Err(VpeError::Runtime(RuntimeError::NoTransitionFound {
                state: current_state,
                action: current_action,
            }));
        }

        let mut matched = None;

        for transition in candidates {
            let passed = transition
                .guards
                .iter()
                .all(|guard| guard.check(&request.context, &request.chronicle.events));

            if passed {
                matched = Some(transition);
                break;
            }
        }

        let transition = match matched {
            Some(transition) => transition,
            None => {
                if current_action == "AUTO_TICK" {
                    return Ok(VpeVerdict {
                        process: request.process.clone(),
                        trace_id: request.trace_id.clone(),
                        previous_state: previous_state_for_verdict,
                        next_state: current_state,
                        state_patch: Default::default(),
                        effects: accumulated_effects,
                        emitted_events,
                    });
                }

                return Err(VpeError::Runtime(RuntimeError::NoTransitionFound {
                    state: current_state.clone(),
                    action: current_action.clone(),
                }));
            }
        };

        let next_state = process.nodes()[transition.target_idx].name.clone();

        accumulated_effects.extend(transition.effects.clone());
        emitted_events.push(PlannedEvent {
            event_kind: VpeEventKind::StateTransition,
            action: current_action.clone(),
            state_before: current_state.clone(),
            state_after: next_state.clone(),
            metadata: json!({}),
        });

        current_state = next_state;

        let next_state_idx = process
            .state_index(&current_state)
            .ok_or_else(|| VpeError::Runtime(RuntimeError::UnknownState(current_state.clone())))?;
        let next_node = &process.nodes()[next_state_idx];

        let has_auto_tick = next_node.transitions.iter().any(|t| t.action == "AUTO_TICK");

        if !has_auto_tick {
            return Ok(VpeVerdict {
                process: request.process.clone(),
                trace_id: request.trace_id.clone(),
                previous_state: previous_state_for_verdict,
                next_state: current_state,
                state_patch: Default::default(),
                effects: accumulated_effects,
                emitted_events,
            });
        }

        auto_depth += 1;
        if auto_depth > MAX_AUTO_DEPTH {
            return Err(VpeError::Runtime(RuntimeError::AutoTransitionLimitExceeded));
        }

        current_action = "AUTO_TICK".to_string();
    }
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
    
    fn law_with_auto_tick() -> LawSource {
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
                        to: "PendingPayment".into(),
                        priority: 100,
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
                    transitions: vec![TransitionSource {
                        action: "AUTO_TICK".into(),
                        to: "PaymentTimeout".into(),
                        priority: 100,
                        guards: vec![GuardSource {
                            guard_type: "TimeElapsed".into(),
                            params: BTreeMap::from([
                                ("seconds".into(), json!(300)),
                            ]),
                        }],
                        effects: vec![],
                        comment: None,
                    }],
                },
                StateSource {
                    name: "PaymentTimeout".into(),
                    is_transient: false,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
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
    
    #[test]
    fn evaluate_fails_when_required_context_missing() {
        let compiled = compiler().compile(&schema(), &law()).unwrap().process;
    
        // Build request WITHOUT rec.amount
        let mut context = ContextMap::new();
        context.insert("sys.now".into(), json!(1_700_000_100_i64));
    
        let anchor_event = anchor("Draft");
    
        let req = VpeRequest {
            process: ProcessRef::new("TestDomain", "TestProcess", "1.0.0"),
            trace_id: "trace-1".into(),
            now: 1_700_000_100,
            current_state: "Draft".into(),
            action: "Submit".into(),
            context,
            chronicle: ChronicleView {
                anchor: anchor_event.clone(),
                events: vec![anchor_event],
            },
        };
    
        let err = evaluate(&compiled, &req).unwrap_err();
    
        assert!(matches!(
            err,
            VpeError::Runtime(RuntimeError::MissingContextField { .. })
        ));
    }
    
    #[test]
    fn evaluate_auto_ticks_into_next_state_when_guard_passes() {
        let compiled = compiler().compile(&schema(), &law_with_auto_tick()).unwrap().process;
        let req = request("Draft", "Submit", 150.0);

        let verdict = evaluate(&compiled, &req).unwrap();

        assert_eq!(verdict.previous_state, "Draft");
        assert_eq!(verdict.next_state, "PaymentTimeout");
        assert_eq!(verdict.emitted_events.len(), 2);
        assert_eq!(verdict.emitted_events[0].state_after, "PendingPayment");
        assert_eq!(verdict.emitted_events[1].state_after, "PaymentTimeout");
    }

    #[test]
    fn evaluate_stops_before_auto_tick_when_guard_does_not_pass() {
        let compiled = compiler().compile(&schema(), &law_with_auto_tick()).unwrap().process;
        let mut req = request("Draft", "Submit", 150.0);
        req.context.insert("sys.now".into(), json!(1_700_000_100_i64));
        req.chronicle.events[0].timestamp = 1_700_000_000;

        let verdict = evaluate(&compiled, &req).unwrap();

        assert_eq!(verdict.next_state, "PendingPayment");
        assert_eq!(verdict.emitted_events.len(), 1);
    }

    #[test]
    fn evaluate_returns_no_transition_found_when_auto_tick_has_no_matching_guard() {
        let compiled = compiler().compile(&schema(), &law_with_auto_tick()).unwrap().process;
        let mut req = request("Draft", "Submit", 150.0);
        req.context.insert("sys.now".into(), json!(1_700_000_100_i64));
        req.chronicle.events[0].timestamp = 1_700_000_050;

        let verdict = evaluate(&compiled, &req).unwrap();

        assert_eq!(verdict.next_state, "PendingPayment");
        assert_eq!(verdict.emitted_events.len(), 1);
    }
}