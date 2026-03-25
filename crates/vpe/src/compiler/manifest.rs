use crate::compiler::source::LawSource;
use crate::types::{HistoryRequirement, StateManifest};
use std::collections::HashMap;

pub fn build_manifests(law: &LawSource) -> HashMap<String, StateManifest> {
    let mut manifests = HashMap::new();

    for state in &law.states {
        manifests.insert(
            state.name.clone(),
            StateManifest {
                history_requirements: vec![HistoryRequirement::LastTransition],
                context_requirements: vec![],
            },
        );
    }

    manifests
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::source::{LawSource, StateSource};

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
}
