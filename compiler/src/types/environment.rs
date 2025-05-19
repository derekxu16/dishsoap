use dishsoap_parser::ast::{
    ClassDeclaration, Declaration, FunctionType, Node, RecordType, Type, TypeReference,
    UntypedNodeCommonFields,
};
use std::{cell::RefCell, collections::HashMap, iter::FromIterator, rc::Rc, ops::Index};

pub type Environment = HashMap<String, Type>;

type TypeReferenceToRecordTypeConverter = dyn Fn(&TypeReference) -> Box<Type>;
pub struct TypeEnvironment {
    pub type_reference_to_record_type_converters:
        HashMap<String, Rc<TypeReferenceToRecordTypeConverter>>,
}

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
    // TODO(derekxu16): Add `extern func`s to the language instead.
    let mut environment: Environment = HashMap::from([
        (
            "__malloc".to_string(),
            Type::FunctionType(Rc::new(FunctionType::new(
                vec![Type::I64Type],
                Type::I64Type,
            )))
        ),
        (
            "__free".to_string(),
            Type::FunctionType(Rc::new(FunctionType::new(
                vec![Type::I64Type],
                Type::UnitType,
            )))
        ),
        (
            "__memMove".to_string(),
            Type::FunctionType(Rc::new(FunctionType::new(
                vec![Type::I64Type, Type::I64Type, Type::I64Type],
                Type::UnitType,
            )))
        ),
        (
            "__memStore".to_string(),
            Type::FunctionType(Rc::new(FunctionType::new(
                vec![Type::I64Type, Type::I64Type, Type::I64Type],
                Type::UnitType,
            )))
        ),
        (
            "__memLoad".to_string(),
            Type::FunctionType(Rc::new(FunctionType::new(
                vec![Type::I64Type, Type::I64Type],
                Type::I64Type,
            )))
        ),
    ]);
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

pub fn populate_type_environment_from_top_level_declarations(
    type_reference_to_record_type_converters: Rc<
        RefCell<HashMap<String, Rc<TypeReferenceToRecordTypeConverter>>>,
    >,
    class_name_to_declaration: Rc<RefCell<HashMap<String, ClassDeclaration>>>,
    source_file_node: &Node<UntypedNodeCommonFields>,
) -> () {
    fn generate_type_reference_to_record_type_converter(
        type_reference_to_record_type_converters: Rc<
            RefCell<HashMap<String, Rc<TypeReferenceToRecordTypeConverter>>>,
        >,
        class_name_to_declaration: Rc<RefCell<HashMap<String, ClassDeclaration>>>,
        class_name: String,
    ) -> Rc<TypeReferenceToRecordTypeConverter> {
        let type_reference_to_record_type_converters_copy =
            type_reference_to_record_type_converters.clone();

        if !type_reference_to_record_type_converters
            .clone()
            .borrow()
            .contains_key(&class_name)
        {
            let class_declaration = class_name_to_declaration
                .borrow_mut()
                .get(&class_name)
                .unwrap()
                .clone();

            type_reference_to_record_type_converters
                .clone()
                .borrow_mut()
                .insert(
                    class_name.clone(),
                    Rc::new(move |t: &TypeReference| {
                        Box::new(Type::RecordType(Rc::new(RecordType::new(
                            HashMap::from_iter(class_declaration.fields.iter().map(
                                |(field_name, field_type)| {
                                    (
                                        (*field_name).clone(),
                                        match field_type {
                                            Type::TypeReference(field_type) => {
                                                if class_declaration.type_parameters
                                                    .contains(&(**field_type).identifier)
                                                {
                                                    match &t.type_arguments.index(class_declaration
                                                        .type_parameters
                                                        .iter()
                                                        .position(|tp| 
                                                            tp.name
                                                                == (**field_type).identifier.name
                                                        )
                                                        .unwrap())
                                                    {
                                                        Type::TypeReference(tr) => {
                                                            *(*generate_type_reference_to_record_type_converter(
                                                                type_reference_to_record_type_converters
                                                                .clone(),class_name_to_declaration.clone(),
                                                                        (**field_type).identifier.name.clone()))(tr)
                                                        }
                                                        argument_type => (*argument_type).clone(),
                                                    }
                                                } else {
                                                    *(*generate_type_reference_to_record_type_converter(
                                                        type_reference_to_record_type_converters.clone(),
                                                        class_name_to_declaration.clone(),
                                                        (**field_type).identifier.name.clone(),
                                                    ))(t)
                                                }
                                            }
                                            _ => (*field_type).clone(),
                                        },
                                    )
                                },
                            )),
                        ))))
                    }),
                );
        }

        let converter = (*type_reference_to_record_type_converters_copy
            .borrow_mut()
            .get(&class_name)
            .unwrap())
        .clone();
        converter
    }

    match source_file_node {
        Node::SourceFile(source_file) => {
            for td in source_file.type_declarations.iter() {
                generate_type_reference_to_record_type_converter(
                    type_reference_to_record_type_converters.clone(),
                    class_name_to_declaration.clone(),
                    td.identifier.name.clone(),
                );
            }
        }
        _ => unreachable!(),
    }
}
