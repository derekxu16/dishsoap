use crate::{Expression, InfixOperator, Type, TypedNodeCommonFields, UntypedNodeCommonFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpression<CommonFields: Clone> {
    pub common_fields: CommonFields,
    pub left: Expression<CommonFields>,
    pub operator: InfixOperator,
    pub right: Expression<CommonFields>,
}

impl BinaryExpression<UntypedNodeCommonFields> {
    pub fn new(
        left: Expression<UntypedNodeCommonFields>,
        operator: InfixOperator,
        right: Expression<UntypedNodeCommonFields>,
    ) -> Self {
        BinaryExpression::<UntypedNodeCommonFields> {
            common_fields: UntypedNodeCommonFields::new(),
            left,
            operator,
            right,
        }
    }
}
impl BinaryExpression<TypedNodeCommonFields> {
    pub fn new(
        r#type: Type,
        left: Expression<TypedNodeCommonFields>,
        operator: InfixOperator,
        right: Expression<TypedNodeCommonFields>,
    ) -> Self {
        BinaryExpression::<TypedNodeCommonFields> {
            common_fields: TypedNodeCommonFields::new(r#type),
            left,
            operator,
            right,
        }
    }
}
