use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Variable {
    pub offset: usize,
}

pub struct Environment {
    pub store: HashMap<String, Rc<Variable>>,
    pub offset: usize,
    pub label_count: usize,
    pub stack: usize,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(offset: usize, stack: usize, label_count: usize) -> Environment {
        Environment {
            store: HashMap::new(),
            offset,
            stack,
            label_count,
            outer: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<Variable>> {
        let stack = self.stack;
        self.get_with_stack(name, stack)
    }

    pub fn get_with_stack(&self, name: &str, stack: usize) -> Option<Rc<Variable>> {
        match self.store.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(out_env) => {
                    if out_env.borrow().stack == stack {
                        out_env.borrow_mut().get_with_stack(name, stack)
                    } else {
                        None
                    }
                }
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str) {
        self.offset += 8;
        self.store.insert(
            name.to_string(),
            Rc::new(Variable {
                offset: self.offset,
            }),
        );
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn inc_label_count(&mut self) -> usize {
        self.label_count += 1;
        self.label_count
    }

    pub fn new_block_env(outer: Rc<RefCell<Environment>>) -> Environment {
        let mut env = Environment::new(
            outer.borrow().offset,
            outer.borrow().stack,
            outer.borrow().label_count,
        );
        env.outer = Some(outer);
        env
    }

    // pub fn new_fn_env(outer: Rc<RefCell<Environment>>) -> Environment {
    //     let mut env = Environment::new(0, outer.borrow().stack + 1, outer.borrow().label_count);
    //     env.outer = Some(outer);
    //     env
    // }
}
