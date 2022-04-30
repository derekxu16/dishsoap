use crate::types::environment::Environment;
use dishsoap_parser::ast::{Declaration, FunctionType, Node, Type, UntypedNodeCommonFields};
use std::{collections::HashMap, rc::Rc};

pub fn gather_top_level_declarations(
    source_file_node: &Node<UntypedNodeCommonFields>,
) -> Environment {
    let mut environment: Environment = HashMap::new();
    match source_file_node {
        Node::SourceFile(source_file) => {
            source_file
                .declarations
                .iter()
                .for_each(|d| match d.as_ref() {
                    Declaration::FunctionDeclaration(fd) => {
                        let parameter_types = fd
                            .parameters
                            .iter()
                            .map(|p| p.variable_declarator.variable_type.clone())
                            .collect();
                        environment.insert(
                            fd.identifier.name.clone(),
                            Type::FunctionType(Rc::new(FunctionType::new(
                                parameter_types,
                                fd.return_type.clone(),
                            ))),
                        );
                    }
                    _ => unreachable!(),
                })
        }
        _ => unreachable!(),
    }

    environment
}
