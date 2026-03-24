use crate::error::SchemaError;
use crate::schema::DomainSchema;
use std::collections::HashSet;

pub fn validate_schema(schema: &DomainSchema) -> Result<(), SchemaError> {
    let mut seen = HashSet::new();

    for field in &schema.fields {
        if !seen.insert(field.name.clone()) {
            return Err(SchemaError::Invalid(format!(
                "duplicate field '{}'",
                field.name
            )));
        }
    }

    Ok(())
}
