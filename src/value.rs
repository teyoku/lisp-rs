use core::fmt;
use std::rc::Rc;

use crate::callable::{BuiltinFunction, Lambda};

#[derive(Clone)]
/// A value in the Lisp runtime.
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(Rc<str>),
    Symbol(Rc<str>),
    List(Rc<Vec<Value>>),

    BuiltinFunction(BuiltinFunction),
    Lambda(Rc<Lambda>),

    Nil,
}

impl Value {
    /// Returns whether this value is treated as true in a conditional.
    ///
    /// Only `#f` is false; all other values, including `Nil`, are truthy.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(val) => *val,
            _ => true,
        }
    }

    /// Returns the Lisp type name of this value.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Boolean(_) => "boolean",
            Value::String(_) => "string",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::BuiltinFunction(_) => "builtin:+",
            Value::Lambda(_) => "lambda",

            Value::Nil => "nil",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(val) => write!(f, "{val}"),
            Value::Float(val) => write!(f, "{val}"),
            Value::Boolean(val) => {
                write!(f, "{}", if *val { "#t" } else { "#f" })
            }
            Value::String(val) => write!(f, "{val}"),
            Value::Symbol(val) => write!(f, "{val}"),
            Value::List(values) => {
                write!(f, "(")?;
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{value}")?;
                }
                write!(f, ")")
            }
            Value::BuiltinFunction(_) => write!(f, "<builtin:+>"),
            Value::Lambda(_) => write!(f, "<lambda>"),
            Value::Nil => write!(f, "()"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(value) => f.debug_tuple("Integer").field(value).finish(),
            Value::Float(value) => f.debug_tuple("Float").field(value).finish(),
            Value::Boolean(value) => f.debug_tuple("Boolean").field(value).finish(),
            Value::String(value) => f.debug_tuple("String").field(value).finish(),
            Value::Symbol(value) => f.debug_tuple("Symbol").field(value).finish(),
            Value::List(values) => f.debug_tuple("List").field(values).finish(),
            Value::BuiltinFunction(_) => f.write_str("BuiltinFunction"),
            Value::Lambda(_) => f.write_str("Lambda"),
            Value::Nil => f.write_str("Nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Symbol(l0), Self::Symbol(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
