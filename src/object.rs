use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Keyword(String),
    BinaryOp(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Keyword(s) => write!(f, "{s}"),
            Object::BinaryOp(s) => write!(f, "{s}"),
            Object::Integer(n) => write!(f, "{n}"),
            Object::Float(n) => write!(f, "{n}"),
            Object::Bool(b) => write!(f, "{b}"),
            Object::String(s) => write!(f, "{s}"),
            Object::Symbol(s) => write!(f, "{s}"),
            Object::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{obj}")?;
                }
                write!(f, ")")
            }
        }
    }
}
