use std::{collections::HashMap, rc::Rc, cell::RefCell};

pub struct Variable {
    pub offset: usize,
}

pub struct Environment {
    pub store: HashMap<String, Variable>,
    pub offset: usize,
    pub label_count: usize,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            offset: 0,
            label_count: 0,
            outer: None,
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&Variable> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str) {
        self.offset += 8;
        self.store.insert(name.to_string(), Variable { offset: self.offset });
    }

    pub fn contains_key(&mut self, name: &str) -> bool {
        self.store.contains_key(name)
    }
}