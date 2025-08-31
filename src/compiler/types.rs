use std::fmt::Display;

use crate::value::objects::FunctionObject;

pub enum FunctionType {
    Function(Box<FunctionObject>),
    Script(Box<FunctionObject>),
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Script(s) => {
                write!(f, "Top-Level Script: {}", s)
            }
            Self::Function(fun) => {
                write!(f, "Function: {fun}")
            }
        }
    }
}

impl FunctionType {
    pub fn default_function() -> Self {
        Self::Function(Box::new(FunctionObject::new()))
    }

    pub fn default_script() -> Self {
        Self::Script(Box::new(FunctionObject::new()))
    }

    pub fn is_script(&self) -> bool {
        match self {
            Self::Script(_) => true,
            _ => false,
        }
    }
    pub fn is_function(&self) -> bool {
        match self {
            Self::Function(_) => true,
            _ => false,
        }
    }
}

impl Into<FunctionObject> for FunctionType {
    fn into(self) -> FunctionObject {
        match self {
            FunctionType::Function(fun) => *fun,
            FunctionType::Script(script) => *script,
        }
    }
}

impl From<FunctionObject> for FunctionType {
    fn from(value: FunctionObject) -> Self {
        FunctionType::Function(Box::new(value))
    }
}
