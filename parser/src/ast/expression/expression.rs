use std::rc::Rc;

use crate::{
    BinaryExpression, BooleanLiteral, FunctionCall, IfExpression, IntegerLiteral, PrefixExpression,
    Type, TypedNodeCommonFields, UnitLiteral, VariableReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<CommonFields: Clone> {
    UnitLiteral(Rc<UnitLiteral<CommonFields>>),
    BooleanLiteral(Rc<BooleanLiteral<CommonFields>>),
    IntegerLiteral(Rc<IntegerLiteral<CommonFields>>),
    VariableReference(Rc<VariableReference<CommonFields>>),
    FunctionCall(Rc<FunctionCall<CommonFields>>),
    PrefixExpression(Rc<PrefixExpression<CommonFields>>),
    BinaryExpression(Rc<BinaryExpression<CommonFields>>),
    IfExpression(Rc<IfExpression<CommonFields>>),
}

impl Expression<TypedNodeCommonFields> {
    pub fn get_type(&self) -> &Type {
        match self {
            Expression::UnitLiteral(u) => &u.common_fields.r#type,
            Expression::BooleanLiteral(b) => &b.common_fields.r#type,
            Expression::IntegerLiteral(i) => &i.common_fields.r#type,
            Expression::VariableReference(r) => &r.common_fields.r#type,
            Expression::FunctionCall(c) => &c.common_fields.r#type,
            Expression::PrefixExpression(e) => &e.common_fields.r#type,
            Expression::BinaryExpression(e) => &e.common_fields.r#type,
            Expression::IfExpression(e) => &e.common_fields.r#type,
        }
    }
}