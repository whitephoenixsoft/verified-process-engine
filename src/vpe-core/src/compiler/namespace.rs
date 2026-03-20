

#[derive(Debug, PartialEq)]
pub enum NamespaceCategory {
    System,   // sys.* (Hardcoded logic)
    Record,   // rec.* (Dynamic schema)
    External, // ext.* (Dynamic schema)
    Calc,     // calc.* (Dynamic schema)
}

impl NamespaceCategory {
    fn from_prefix(prefix: &str) -> Result<Self, String> {
        match prefix {
            "sys"  => Ok(NamespaceCategory::System),
            "rec"  => Ok(NamespaceCategory::Record),
            "ext"  => Ok(NamespaceCategory::External),
            "calc" => Ok(NamespaceCategory::Calc),
            _      => Err(format!("Unknown namespace prefix: '{}'", prefix)),
        }
    }
}