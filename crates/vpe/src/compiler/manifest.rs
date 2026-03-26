use crate::compiler::source::LawSource;
use crate::types::{ContextRequirement, HistoryRequirement, StateManifest};
use std::collections::HashMap;

pub fn build_manifests(law: &LawSource) -> HashMap<String, StateManifest> {
    let mut manifests = HashMap::new();

    for state in &law.states {
        let mut context_requirements = Vec::new();

        for transition in &state.transitions {
            for guard in &transition.guards {
                if guard.guard_type == "GreaterThan" {
                    if let Some(path) = guard.params.get("path").and_then(|v| v.as_str()) {
                        let req = ContextRequirement::Field(path.to_string());
                        if !context_requirements.contains(&req) {
                            context_requirements.push(req);
                        }
                    }
                }
            }
        }

        manifests.insert(
            state.name.clone(),
            StateManifest {
                history_requirements: vec![HistoryRequirement::LastTransition],
                context_requirements,
            },
        );
    }

    manifests
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::source::{GuardSource, LawSource, StateSource, TransitionSource};
    use crate::types::ContextRequirement;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn builds_manifest_for_each_state() {
        let law = LawSource {
            domain: "TestDomain".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![
                StateSource {
                    name: "Draft".into(),
                    is_transient: false,
                    transitions: vec![],
                },
                StateSource {
                    name: "Approved".into(),
                    is_transient: false,
                    transitions: vec![],
                },
            ],
            migration_rules: vec![],
        };

        let manifests = build_manifests(&law);

        assert_eq!(manifests.len(), 2);
        assert!(manifests.contains_key("Draft"));
        assert!(manifests.contains_key("Approved"));
    }

    #[test]
    fn always_includes_last_transition_requirement() {
        let law = LawSource {
            domain: "TestDomain".into(),
            process: "TestProcess".into(),
            version: "1.0.0".into(),
            initial_state: "Draft".into(),
            states: vec![StateSource {
                name: "Draft".into(),
                is_transient: false,
                transitions: vec![],
            }],
            migration_rules: vec![],
        };

        let manifests = build_manifests(&law);
        let manifest = manifests.get("Draft").unwrap();

        assert_eq!(
            manifest.history_requirements,
            vec![HistoryRequirement::LastTransition]
        );
    }

    #[test]
    fn includes_context_requirement_for_greater_than_guard() {
        let law = LawSource {
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
                            guard_type: "GreaterThan".into(),
                            params: BTreeMap::from([
                                ("path".into(), json!("rec.amount")),
                                ("value".into(), json!(100)),
                            ]),
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

        let manifests = build_manifests(&law);
        let manifest = manifests.get("Draft").unwrap();

        assert!(manifest
            .context_requirements
            .contains(&ContextRequirement::Field("rec.amount".into())));
    }
}
