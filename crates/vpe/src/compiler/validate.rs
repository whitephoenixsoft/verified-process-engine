use crate::compiler::source::LawSource;
use crate::error::CompileError;
use crate::schema::{validate_schema, DomainSchema};

pub fn validate_law(schema: &DomainSchema, law: &LawSource) -> Result<(), CompileError> {
    validate_schema(schema).map_err(crate::error::VpeError::from).map_err(|e| match e {
        crate::error::VpeError::Schema(se) => CompileError::InvalidLaw(se.to_string()),
        other => CompileError::InvalidLaw(other.to_string()),
    })?;

    if law.states.is_empty() {
        return Err(CompileError::InvalidLaw(
            "law must contain at least one state".into(),
        ));
    }

    Ok(())
}
