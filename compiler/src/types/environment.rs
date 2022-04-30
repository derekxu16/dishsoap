use dishsoap_parser::ast::Type;
use std::collections::HashMap;

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
