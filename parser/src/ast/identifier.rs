#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Identifier {
        Identifier { name }
    }
}
