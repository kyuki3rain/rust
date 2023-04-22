use std::collections::HashMap;

pub struct Variable {
    pub offset: usize,
}

pub struct Environment {
    pub store: HashMap<String, Variable>,
    pub offset: usize,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            offset: 0,
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Environment) -> Environment {
        return Environment {
            store: HashMap::new(),
            offset: 0,
            outer: Some(Box::new(outer)),
        };
    }

    pub fn close_environment(self) -> Environment{
        if let Some(env) = self.outer {
            return *env;
        } else {
            panic!("No outer environment");
        }
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str) {
        self.offset += 8;
        self.store.insert(name.to_string(), Variable { offset: self.offset });
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }
}