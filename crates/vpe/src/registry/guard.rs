use crate::types::{ContextMap, GuardRequirements, VpeEvent};
use std::fmt::Debug;

pub trait Guard: Send + Sync + Debug {
    fn check(&self, context: &ContextMap, history: &[VpeEvent]) -> bool;
    fn requirements(&self) -> GuardRequirements {
        GuardRequirements::default()
    }
    fn name(&self) -> &'static str;
}
