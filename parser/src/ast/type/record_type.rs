use std::collections::HashMap;

use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordType {
    pub fields: HashMap<String, Type>,
}

impl RecordType {
    pub fn new(fields: HashMap<String, Type>) -> Self {
        RecordType { fields }
    }
}
