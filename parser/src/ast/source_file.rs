use std::rc::Rc;

use crate::Declaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile<CommonFields: Clone> {
    pub declarations: Vec<Rc<Declaration<CommonFields>>>,
}

impl<CommonFields: Clone> SourceFile<CommonFields> {
    pub fn new(declarations: Vec<Rc<Declaration<CommonFields>>>) -> Self {
        SourceFile::<CommonFields> { declarations }
    }
}
