/// A separate value type for our data to be stored on VM
// pub type Value = f64;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Value {

    pub fn is_bool(&self) -> bool {
        match &self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match &self {
            Self::Nil => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match &self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_falsey(self) -> bool {
        self.is_nil() || (self.is_bool() && !(Into::<bool>::into(self)))
    }

    pub fn to_number(self) -> f64 {
        self.into()
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Self::Bool(b) => b,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Self::Number(n) => n,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl Add for Value {
    type Output = self::Value;
    fn add(self, rhs: Self) -> Self::Output {
        if let Self::Number(a) = self
            && let Self::Number(b) = rhs
        {
            return Self::Number(a + b);
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

impl Sub for Value {
    type Output = self::Value;
    fn sub(self, rhs: Self) -> Self::Output {
        if let Self::Number(a) = self
            && let Self::Number(b) = rhs
        {
            return Self::Number(a - b);
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

impl Mul for Value {
    type Output = self::Value;
    fn mul(self, rhs: Self) -> Self::Output {
        if let Self::Number(a) = self
            && let Self::Number(b) = rhs
        {
            return Self::Number(a * b);
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

impl Div for Value {
    type Output = self::Value;
    fn div(self, rhs: Self) -> Self::Output {
        if let Self::Number(a) = self
            && let Self::Number(b) = rhs
        {
            return Self::Number(a / b);
        }

        // This should be unreachable, types should be checked in compiler for proper
        // error handling
        unreachable!()
    }
}

impl Neg for Value {
    type Output = self::Value;

    fn neg(self) -> Self::Output {
        if let Self::Number(a) = self {
            return Self::Number(-a);
        }
        // This code shouldn't be reached
        unreachable!()
    }
}

impl Not for Value {
    type Output = self::Value;
    fn not(self) -> Self::Output {
        if let Self::Bool(b) = self {
            return Self::Bool(!b);
        }

        // This code shouldn't be reached
        unreachable!();
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => {
                write!(f, "Nil")
            }
            Self::Bool(v) => {
                write!(f, "{v}")
            }
            Self::Number(n) => {
                write!(f, "{n}")
            }
        }
    }
}
