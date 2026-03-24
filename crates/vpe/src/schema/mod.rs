pub mod domain_schema;
pub mod types;
pub mod validation;

pub use domain_schema::DomainSchema;
pub use types::{FieldDefinition, SchemaFieldType};
pub use validation::validate_schema;
pub use crate::compiler::source::LawSource;
