use super::{Expression, Statement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<CommonFields: Clone> {
    pub statements: Vec<Statement<CommonFields>>,
    pub final_expression: Option<Expression<CommonFields>>,
}

impl<CommonFields: Clone> Block<CommonFields> {
    pub fn new(
        statements: Vec<Statement<CommonFields>>,
        final_expression: Option<Expression<CommonFields>>,
    ) -> Self {
        Block::<CommonFields> {
            statements,
            final_expression,
        }
    }

    pub fn new_no_final_expression(statements: Vec<Statement<CommonFields>>) -> Self {
        Block::<CommonFields> {
            statements,
            final_expression: None,
        }
    }

    pub fn new_with_final_expression(
        statements: Vec<Statement<CommonFields>>,
        final_expression: Expression<CommonFields>,
    ) -> Self {
        Block::<CommonFields> {
            statements,
            final_expression: Some(final_expression),
        }
    }
}
