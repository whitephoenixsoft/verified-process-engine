use super::Guard;
use super::ContextMap;
use super::VpeEvent;
use serde_json::Value;

pub struct EqualsGuard {
    pub path: String,
    pub expected: Value,
}

impl Guard for EqualsGuard {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool {
        context.get(&self.path)
            .map(|val| val == &self.expected)
            .unwrap_or(false)
    }

    fn get_requirements(&self) -> Vec<HistoryRequirement> {
        vec![
            HistoryRequirement::FieldDependency(path), // Needs to host to feed it a specific schema value specified in the JSON
        ] 
    }
}


