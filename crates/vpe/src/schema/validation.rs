
use crate::error::SchemaError;
use crate::schema::{DomainSchema, FieldDefinition, SchemaFieldType};
use std::collections::HashSet;

pub fn validate_schema(schema: &DomainSchema) -> Result<(), SchemaError> {
    validate_namespace_fields("rec", &schema.namespaces.rec)?;
    validate_namespace_fields("ext", &schema.namespaces.ext)?;
    validate_namespace_fields("calc", &schema.namespaces.calc)?;
    Ok(())
}

fn validate_namespace_fields(
    namespace: &str,
    fields: &[FieldDefinition],
) -> Result<(), SchemaError> {
    if namespace == "sys" {
        return Err(SchemaError::Invalid(
            "sys namespace may not be user-defined".into(),
        ));
    }

    let mut seen = HashSet::new();

    for field in fields {
        validate_identifier(&field.name)?;

        if !seen.insert(field.name.clone()) {
            return Err(SchemaError::Invalid(format!(
                "duplicate field '{}' in namespace '{}'",
                field.name, namespace
            )));
        }

        validate_field_definition(field)?;
    }

    Ok(())
}

fn validate_field_definition(field: &FieldDefinition) -> Result<(), SchemaError> {
    match field.field_type {
        SchemaFieldType::Enum => {
            let values = field.enum_values.as_ref().ok_or_else(|| {
                SchemaError::Invalid(format!(
                    "enum field '{}' must define enum_values",
                    field.name
                ))
            })?;

            if values.is_empty() {
                return Err(SchemaError::Invalid(format!(
                    "enum field '{}' must define at least one enum value",
                    field.name
                )));
            }

            let mut seen = HashSet::new();
            for value in values {
                validate_identifier(value)?;
                if !seen.insert(value.clone()) {
                    return Err(SchemaError::Invalid(format!(
                        "enum field '{}' contains duplicate value '{}'",
                        field.name, value
                    )));
                }
            }
        }
        _ => {
            if field.enum_values.is_some() {
                return Err(SchemaError::Invalid(format!(
                    "field '{}' defines enum_values but is not of type Enum",
                    field.name
                )));
            }
        }
    }

    Ok(())
}

fn validate_identifier(value: &str) -> Result<(), SchemaError> {
    let mut chars = value.chars();

    let first = chars.next().ok_or_else(|| {
        SchemaError::Invalid("identifier may not be empty".into())
    })?;

    if first.is_ascii_digit() {
        return Err(SchemaError::Invalid(format!(
            "identifier '{}' may not start with a digit",
            value
        )));
    }

    if !(first.is_ascii_alphanumeric() || first == '_') {
        return Err(SchemaError::Invalid(format!(
            "identifier '{}' contains invalid characters",
            value
        )));
    }

    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(SchemaError::Invalid(format!(
            "identifier '{}' contains invalid characters",
            value
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{DomainSchema, SchemaNamespaces};

    #[test]
    fn validates_namespaced_schema() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![FieldDefinition {
                    name: "amount".into(),
                    field_type: SchemaFieldType::Number,
                    description: None,
                    enum_values: None,
                }],
                ext: vec![],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_ok());
    }

    #[test]
    fn rejects_invalid_field_identifier() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![FieldDefinition {
                    name: "order total".into(),
                    field_type: SchemaFieldType::Number,
                    description: None,
                    enum_values: None,
                }],
                ext: vec![],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn rejects_duplicate_field_in_same_namespace() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![
                    FieldDefinition {
                        name: "amount".into(),
                        field_type: SchemaFieldType::Number,
                        description: None,
                        enum_values: None,
                    },
                    FieldDefinition {
                        name: "amount".into(),
                        field_type: SchemaFieldType::Number,
                        description: None,
                        enum_values: None,
                    },
                ],
                ext: vec![],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn allows_same_field_name_in_different_namespaces() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![FieldDefinition {
                    name: "status".into(),
                    field_type: SchemaFieldType::String,
                    description: None,
                    enum_values: None,
                }],
                ext: vec![FieldDefinition {
                    name: "status".into(),
                    field_type: SchemaFieldType::String,
                    description: None,
                    enum_values: None,
                }],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_ok());
    }

    #[test]
    fn rejects_enum_without_values() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![FieldDefinition {
                    name: "status".into(),
                    field_type: SchemaFieldType::Enum,
                    description: None,
                    enum_values: None,
                }],
                ext: vec![],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_err());
    }

    #[test]
    fn rejects_duplicate_enum_values() {
        let schema = DomainSchema {
            domain: "TestDomain".into(),
            version: "1.0.0".into(),
            namespaces: SchemaNamespaces {
                rec: vec![FieldDefinition {
                    name: "status".into(),
                    field_type: SchemaFieldType::Enum,
                    description: None,
                    enum_values: Some(vec!["Pending".into(), "Pending".into()]),
                }],
                ext: vec![],
                calc: vec![],
            },
        };

        assert!(validate_schema(&schema).is_err());
    }
}