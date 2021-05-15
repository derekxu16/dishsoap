use dishsoap_parser::ast::Node;

pub fn string_to_c_str(s: &String) -> *const i8 {
    return format!("{}\0", s).as_ptr() as *const i8;
}

pub fn identifier_to_string(identifier: &Node) -> String {
    match identifier {
        Node::Identifier(i) => i.name.to_owned(),
        _ => panic!("Compilation error: expected an identifier"),
    }
}

pub fn identifier_to_c_str(identifier: &Node) -> *const i8 {
    match identifier {
        Node::Identifier(i) => string_to_c_str(&i.name),
        _ => panic!("Compilation error: expected an identifier"),
    }
}
