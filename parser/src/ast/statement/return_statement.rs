use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<CommonFields: Clone> {
    pub expression: Expression<CommonFields>,
}

impl<CommonFields: Clone> ReturnStatement<CommonFields> {
    pub fn new(expression: Expression<CommonFields>) -> Self {
        ReturnStatement::<CommonFields> { expression }
    }
}
