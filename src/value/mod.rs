/// A separate value type for our data to be stored on VM
// pub type Value = f64;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

/// Represents supported types and their values
/// Since value can be of only a single type, enum is enough for now
/// Each variant will take 16 bytes in memory, like boolean will also take 16 bytes
/// because largest variant size is 16 bytes, which is Number(f64).
/// f64 will take 8 bytes, compiler will use 1 byte to store variant information, and rest
/// will be padding, due to alignment.
/// This is certainly not an efficent solution since Bool will also take 16 bytes, actually
/// it's a waste of memory. If we want to optimize in such a way that a boolean should take
/// 1 byte, we've to re-think how to represent Value internally. It will make code much more
/// complex and requires a careful design
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    /// Represents boolean variant which also stores value
    Bool(bool),
    /// Equivalent to `None` or `null`
    Nil,
    /// Numbers are represented as `f64`
    Number(f64),
}

impl Value {
    /// If value is pf boolean type, returns true
    pub fn is_bool(&self) -> bool {
        match &self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    /// If value is nil, returns true
    pub fn is_nil(&self) -> bool {
        match &self {
            Self::Nil => true,
            _ => false,
        }
    }

    /// Returns true if value is a number
    pub fn is_number(&self) -> bool {
        match &self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    /// Used to invert truthy value
    pub fn is_falsey(self) -> bool {
        self.is_nil() || (self.is_bool() && !(Into::<bool>::into(self)))
    }

    /// Just convert `Value` instance to `f64`
    pub fn to_number(self) -> f64 {
        self.into()
    }
}

/// Implements `Into` trait to extract `bool` from `Value::Bool`
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

/// Implements `Into` trait to extract `f64` from `Value::Number`
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

/// Implements `From` trait to convert from `bool` to `Value::Bool`
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// Implements `From` trait to convert from `f64` to `Value::Number`
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

/// Overrides '+' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
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

/// Overrides '-' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
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

/// Overrides '*' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
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

/// Overrides '/' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
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

/// Overrides '-' (negation, which is unary) operator, for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
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
/// Overrides '!' operator for custom type `Value`, only works if value is Bool
/// It's like operator overloading in C++
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

/// Implements Display trait for nicer output
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
