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
                write!(f, "Top-Level Script: {s}")
            }
            Self::Function(fun) => {
                write!(f, "Function: {fun}")
            }
        }
    }
}

impl FunctionType {
    #[must_use]
    pub fn default_function() -> Self {
        Self::Function(Box::default())
    }

    #[must_use]
    pub fn default_script() -> Self {
        Self::Script(Box::default())
    }

    #[must_use]
    pub fn is_script(&self) -> bool {
        matches!(self, Self::Script(_))
    }

    #[must_use]
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }
}

impl From<FunctionType> for FunctionObject {
    fn from(val: FunctionType) -> FunctionObject {
        match val {
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
