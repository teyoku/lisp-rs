use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    error::{LispError, LispErrorKind},
    value::Value,
};

pub type EnvironmentRef = Rc<RefCell<Environment>>;

/// A lexical environment containing bindings and an optional parent scope.
#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Weak<RefCell<Environment>>>,
}

impl Environment {
    /// Creates an empty environment enclosed by `parent`.
    pub fn enclosed(parent: &EnvironmentRef) -> EnvironmentRef {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            parent: Some(Rc::downgrade(parent)),
        }))
    }

    /// Defines or replaces a binding in the local environment.
    pub fn define(environment: &EnvironmentRef, name: impl Into<String>, value: Value) {
        environment.borrow_mut().values.insert(name.into(), value);
    }

    /// Looks up a binding in the environment and its parent scopes.
    pub fn get(environment: &EnvironmentRef, name: &str) -> Result<Value, LispError> {
        let mut current = Rc::clone(environment);

        loop {
            let (value, parent) = {
                let environment = current.borrow();
                (
                    environment.values.get(name).cloned(),
                    environment.parent.as_ref().and_then(Weak::upgrade),
                )
            };

            if let Some(value) = value {
                return Ok(value);
            }

            current = match parent {
                Some(parent) => parent,
                None => {
                    return Err(LispError::new(LispErrorKind::UndefinedVariable(
                        name.to_string(),
                    )));
                }
            };
        }
    }

    /// Assigns a value to the nearest existing binding in the environment chain.
    pub fn assign(environment: &EnvironmentRef, name: &str, value: Value) -> Result<(), LispError> {
        let mut current = Rc::clone(environment);

        loop {
            let parent = {
                let mut environment = current.borrow_mut();
                if environment.values.contains_key(name) {
                    environment.values.insert(name.to_string(), value);
                    return Ok(());
                }
                environment.parent.as_ref().and_then(Weak::upgrade)
            };

            current = match parent {
                Some(parent) => parent,
                None => {
                    return Err(LispError::new(LispErrorKind::UndefinedVariable(
                        name.to_string(),
                    )));
                }
            };
        }
    }

    /// Returns whether a binding exists in the local environment only.
    pub fn contains_local(environment: &EnvironmentRef, name: &str) -> bool {
        environment.borrow().values.contains_key(name)
    }
}
