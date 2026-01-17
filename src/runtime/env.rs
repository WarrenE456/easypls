use std::collections::HashMap;

pub struct Env {
    identifier_to_value: HashMap<String, bool>,
}

impl Env {
    pub fn new() -> Env {
        Env { identifier_to_value: HashMap::new() }
    }

    pub fn define(&mut self, identifier: String, value: bool) {
        self.identifier_to_value.insert(identifier, value);
    }

    pub fn get(&mut self, identifier: &String) -> Option<bool> {
        self.identifier_to_value
            .get(identifier)
            .map(|v| *v)
    }
}
