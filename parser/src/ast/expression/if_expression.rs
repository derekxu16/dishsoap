use std::rc::Rc;

use crate::{Block, Expression, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpression<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub condition: Expression<CommonFields>,
    pub then_block: Rc<Block<CommonFields>>,
    pub else_block: Rc<Block<CommonFields>>,
}

impl IfExpression<UntypedNodeCommonFields> {
    pub fn new(
        condition: Expression<UntypedNodeCommonFields>,
        then_block: Rc<Block<UntypedNodeCommonFields>>,
        else_block: Rc<Block<UntypedNodeCommonFields>>,
    ) -> Self {
        IfExpression::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            condition,
            then_block,
            else_block,
        }
    }
}

impl IfExpression<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        condition: Expression<TypedNodeCommonFields>,
        then_block: Rc<Block<TypedNodeCommonFields>>,
        else_block: Rc<Block<TypedNodeCommonFields>>,
    ) -> Self {
        IfExpression::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            condition,
            then_block,
            else_block,
        }
    }
}
