use core::fmt;

use crate::{ast::Expr, environment::EnvironmentRef, error::LispError, value::Value};

/// A native function callable from Lisp code.
pub type BuiltinFn = fn(&[Value]) -> Result<Value, LispError>;

/// A named native Lisp function.
#[derive(Clone)]
pub struct BuiltinFunction {
    pub name: &'static str,
    pub function: BuiltinFn,
}

/// A Lisp function and the environment captured when it was created.
#[derive(Clone)]
pub struct Lambda {
    pub parameters: Vec<String>,
    pub body: Vec<Expr>,
    pub closure: EnvironmentRef,
}

impl BuiltinFunction {
    /// Creates a built-in function with the given name and implementation.
    pub fn new(name: &'static str, function: BuiltinFn) -> Self {
        Self { name, function }
    }

    /// Calls the built-in function with the supplied arguments.
    pub fn call(&self, arguments: &[Value]) -> Result<Value, LispError> {
        (self.function)(arguments)
    }
}

impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .field("function", &self.function)
            .finish()
    }
}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lambda")
            .field("parameters", &self.parameters)
            .field("body", &self.body)
            .field("closure", &self.closure)
            .finish()
    }
}
