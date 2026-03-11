pub struct VpeRuntime;

pub struct VpeVerdict {
    pub next_state_idx: usize,
    pub next_state_name: String,
    pub effects: Vec<VpeEffect>,
}

pub struct VpeEffect {
    pub effect_type: String,
    pub target: String,
    pub payload: serde_json::Value,
}

impl VpeRuntime {
    pub fn evaluate(
        dag: &VpeDag,
        current_idx: usize,
        action: &str,
        context: &ContextMap,
        history: &[VpeEvent]
    ) -> Result<VpeVerdict, String> {
        // 1. O(1) Lookup: Jump directly to the current node in the arena
        let current_node = dag.nodes.get(current_idx)
            .ok_or_else(|| format!("Runtime Error: State index {} out of bounds", current_idx))?;

        // 2. Find matching edges for the requested action
        // (Compiler should have pre-sorted these by priority)
        for edge in &current_node.transitions {
            if edge.action != action {
                continue;
            }

            // 3. Short-Circuit Guard Evaluation
            // If all guards return true, this is our path.
            let all_guards_pass = edge.guards.iter().all(|guard| {
                guard.check(context, history)
            });

            if all_guards_pass {
                let target_node = &dag.nodes[edge.target_idx];
                
                return Ok(VpeVerdict {
                    next_state_idx: edge.target_idx,
                    next_state_name: target_node.name.clone(),
                    effects: edge.effects.clone(),
                });
            }
        }

        // 4. Fallback: No valid transition found for this action/context
        Err(format!("Access Denied: No valid transition for action '{}' from state '{}'", 
            action, current_node.name))
    }
}
    fn get_validated_start_state(dag: &VpeDag, history: &[VpeEvent]) -> Result<usize, VpeError> {
        // 1. Get the last 'Transition' event from the chronicle
        let last_transition = history.iter()
            .rev()
            .find(|e| e.event_type == "STATE_TRANSITION");

        match last_transition {
            Some(event) => {
                // 2. Cross-reference the state name with the DAG's index
                dag.get_state_idx(&event.to_state)
                    .ok_or(VpeError::HistoryCorruption("State in history not in Law".into()))
            },
            None => {
                // 3. No history? Use the initial state of the DAG
                Ok(dag.initial_state_idx)
            }
        }
    }
}
