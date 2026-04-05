pub mod domain_schema;
pub mod types;
pub mod validation;

pub use validation::validate_schema;
pub use crate::compiler::source::LawSource;
pub use types::{FieldDefinition, SchemaFieldType};
pub use domain_schema::{DomainSchema, SchemaNamespaces};
