use super::Guard;
use super::ContextMap;
use super::VpeEvent;
use serde_json::Value;


pub struct OccurredWithinGuard {
    pub target_action: String,
    pub window_seconds: u64,
}

impl Guard for OccurredWithinGuard {
    fn check(&self, _context: &ContextMap, history: &[VpeEvent]) -> bool {
        // 1. Get the most recent event's time as our "now" 
        // (Ensures determinism regardless of when the code runs)
        let now = match history.last() {
            Some(e) => e.timestamp,
            None => return false,
        };

        // 2. Scan history (backwards for performance)
        history.iter().rev().any(|event| {
            event.action == self.target_action && 
            (now - event.timestamp) <= self.window_seconds
        })
    }

    fn get_requirements(&self) -> Vec<HistoryRequirement> {
        vec![HistoryRequirement::LastEventOfAction(self.target_action)]
    }
}
