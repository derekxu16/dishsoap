use super::{ClassDeclaration, Declaration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile<CommonFields: Clone> {
    pub declarations: Vec<Declaration<CommonFields>>,
    pub type_declarations: Vec<ClassDeclaration>,
}

impl<CommonFields: Clone> SourceFile<CommonFields> {
    pub fn new(
        declarations: Vec<Declaration<CommonFields>>,
        type_declarations: Vec<ClassDeclaration>,
    ) -> Self {
        SourceFile::<CommonFields> {
            declarations,
            type_declarations,
        }
    }
}
