use dishsoap_parser::ast::{
    ClassDeclaration, Declaration, FunctionType, Node, RecordType, Type, TypeReference,
    UntypedNodeCommonFields,
};
use std::{collections::HashMap, iter::FromIterator, rc::Rc};

pub type Environment = HashMap<String, Type>;

pub struct EnvironmentStack {
    stack: Vec<Environment>,
}

impl EnvironmentStack {
    pub fn new(initial_environment: Environment) -> EnvironmentStack {
        EnvironmentStack {
            stack: vec![initial_environment],
        }
    }

    pub fn enter_scope(&mut self) -> &mut Environment {
        self.stack.push(match self.stack.last() {
            Some(e) => HashMap::from(e.clone()),
            None => HashMap::new(),
        });

        self.top()
    }

    pub fn exit_scope(&mut self) -> () {
        self.stack.pop();
    }

    pub fn top(&mut self) -> &mut Environment {
        self.stack.last_mut().unwrap()
    }
}

pub fn build_environment_from_top_level_declarations(
    source_file_node: &Node<UntypedNodeCommonFields>,
) -> Environment {
    let mut environment: Environment = HashMap::new();
    match source_file_node {
        Node::SourceFile(source_file) => source_file.declarations.iter().for_each(|d| match d {
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
        }),
        _ => unreachable!(),
    }

    environment
}

pub fn build_type_environment_from_top_level_declarations(
    source_file_node: &Node<UntypedNodeCommonFields>,
) -> Environment {
    match source_file_node {
        Node::SourceFile(source_file) => {
            let class_name_to_declaration: HashMap<String, &ClassDeclaration> = HashMap::from_iter(
                source_file
                    .type_declarations
                    .iter()
                    .map(|d| (d.identifier.name.clone(), d)),
            );

            fn convert_type_reference_to_record_type(
                class_name_to_declaration: &HashMap<String, &ClassDeclaration>,
                t: &Type,
            ) -> Type {
                match t {
                    Type::TypeReference(r) => {
                        let class_declaration =
                            match class_name_to_declaration.get(&(*r).identifier.name) {
                                Some(c) => *c,
                                None => panic!("Compilation error"),
                            };
                        Type::RecordType(Rc::new(RecordType::new(HashMap::from_iter(
                            class_declaration.fields.iter().map(|(n, t)| {
                                (
                                    (*n).clone(),
                                    convert_type_reference_to_record_type(
                                        class_name_to_declaration,
                                        t,
                                    ),
                                )
                            }),
                        ))))
                    }
                    _ => (*t).clone(),
                }
            }

            HashMap::<String, Type>::from_iter(source_file.type_declarations.iter().map(|d| {
                (
                    d.identifier.name.clone(),
                    convert_type_reference_to_record_type(
                        &class_name_to_declaration,
                        &Type::TypeReference(Rc::new(TypeReference::new(d.identifier.clone()))),
                    ),
                )
            }))
        }
        _ => unreachable!(),
    }
}
