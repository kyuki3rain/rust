use std::collections::HashMap;

pub struct Variable {
    pub offset: usize,
}

pub struct Environment {
    pub store: HashMap<String, Variable>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut store = HashMap::new();

        store.insert(String::from("a"), Variable { offset: 8 });
        
        Environment {
            store,
            outer: None,
        }
    }

    pub fn new_enclosed_environment(outer: Environment) -> Environment {
        return Environment {
            store: HashMap::new(),
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

    pub fn set(&mut self, name: String, variable: Variable) {
        self.store.insert(name, variable);
    }
}