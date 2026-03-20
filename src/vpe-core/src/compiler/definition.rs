
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VpeType {
    String,
    Number,
    Boolean,
    /// Represented as Unix Seconds (u64)
    DateTime,
    /// Represented as Seconds (u64)
    Duration,
    Enum(Vec<String>),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub data_type: VpeType,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaDefinition {
    pub domain: String,
    pub version: String,
    pub fields: HashMap<String, FieldDefinition>,
}

impl SchemaDefinition {
    /// Returns the type of a field if it exists in the 'rec.*' namespace
    pub fn resolve_rec_type(&self, path: &str) -> Option<&VpeType> {
        // Strip 'rec.' prefix if present
        let clean_path = path.strip_prefix("rec.").unwrap_or(path);
        self.fields.get(clean_path).map(|f| &f.data_type)
    }
}
