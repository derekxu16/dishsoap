use std::ffi::CString;

use dishsoap_parser::ast::Identifier;

pub fn string_to_c_string(s: String) -> CString {
    return CString::new(s).unwrap();
}

pub fn identifier_to_string(identifier: &Identifier) -> String {
    identifier.name.clone()
}

pub fn identifier_to_c_string(identifier: &Identifier) -> CString {
    string_to_c_string(identifier.name.clone())
}
