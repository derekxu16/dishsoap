use crate::{Expression, PrefixOperator, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpression<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub operator: PrefixOperator,
    pub operand: Expression<CommonFields>,
}

impl PrefixExpression<UntypedNodeCommonFields> {
    pub fn new(operator: PrefixOperator, operand: Expression<UntypedNodeCommonFields>) -> Self {
        PrefixExpression::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            operator,
            operand,
        }
    }
}

impl PrefixExpression<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        operator: PrefixOperator,
        operand: Expression<TypedNodeCommonFields>,
    ) -> Self {
        PrefixExpression::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            operator,
            operand,
        }
    }
}
