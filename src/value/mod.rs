/// A separate value type for our data to be stored on VM
// pub type Value = f64;
use std::{
    ops::{Add, Div, Mul, Neg, Not, Sub},
    ptr::NonNull,
};

use crate::vm::VM;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    String(Box<String>),
    Other,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                // let new_str = s.replace(' ', "•");
                write!(f, "\"{s}\"")
            }
            Self::Other => {
                write!(f, "OTHER")
            }
        }
    }
}

pub type GCObject = Option<NonNull<Object>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    ty: ObjectType,
    pub next: GCObject,
}

impl Object {
    pub fn new(ty: ObjectType) -> Self {
        Self { ty, next: None }
    }

    /// All runtime objects should be created with this method
    pub fn with_vm(ty: ObjectType, vm: &mut VM) -> Self {
        let objects = vm.objects.take();

        #[cfg(feature = "debug_trace_execution")]
        {
            println!("-------GC Insert---------");
            println!("{ty}");
            println!("-------------------------");
        }

        let obj = Self { ty, next: objects };
        vm.objects = NonNull::new(Box::into_raw(Box::new(obj.clone())));

        obj
    }

    pub fn from_runtime_str(value: String, vm: &mut VM) -> Self {
        Self::with_vm(ObjectType::String(Box::new(value)), vm)
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Self::new(ObjectType::String(Box::new(value)))
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)
    }
}

/// Represents supported types and their values.
/// Since value can be of only a single type, enum is enough for now.
/// Each variant will take 16 bytes in memory, like boolean will also take 16 bytes
/// because largest variant size is 16 bytes, which is Number(f64).
/// f64 will take 8 bytes, compiler will use 1 byte to store variant information, and rest
/// will be padding, due to alignment.
/// This is certainly not an efficent solution since Bool will also take 16 bytes, actually
/// it's a waste of memory. If we want to optimize in such a way that a boolean should take
/// 1 byte, we've to re-think how to represent Value internally. It will make code much more
/// complex and requires a careful design.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Represents boolean variant which also stores value
    Bool(bool),
    /// Equivalent to `null`
    Nil,
    /// Numbers are represented as `f64`
    Number(f64),
    /// Stores pointer to the object stored on heap
    Obj(Box<Object>),
}

impl Value {
    pub fn from_runtime_str(value: String, vm: &mut VM) -> Self {
        Self::Obj(Box::new(Object::from_runtime_str(value, vm)))
    }
    /// If value is pf boolean type, returns true
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// If value is nil, returns true
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    /// Returns true if value is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns true if value is an object
    pub fn is_obj(&self) -> bool {
        matches!(self, Self::Obj(_))
    }

    /// Used to invert truthy value
    pub fn is_falsey(self) -> bool {
        self.is_nil() || (self.is_bool() && !(Into::<bool>::into(self)))
    }

    /// Just convert `Value` instance to `f64`
    pub fn to_number(self) -> f64 {
        self.into()
    }

    /// Just convert `Value` instance to `Object`
    pub fn as_object(self) -> Object {
        self.into()
    }

    pub fn as_object_string(self) -> String {
        self.into()
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::Obj(obj) if matches!((**obj).ty, ObjectType::String(_)))
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

/// Implements `Into` trait to extract `Obj` from `Value::Obj`
impl Into<Object> for Value {
    fn into(self) -> Object {
        match self {
            Self::Obj(n) => *n,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}

/// Implements `Into` trait to extract `Obj` from `Value::Obj`
impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Self::Obj(n) => match (*n).ty {
                ObjectType::String(s) => *s,
                _ => unreachable!(),
            },
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

/// Implements `From` trait to convert from `Object` to `Value::Obj`
impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Obj(Box::new(value))
    }
}

/// Implements `From` trait to convert from `Object` to `Value::Obj`
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Obj(Box::new(Object::from(value)))
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
impl std::fmt::Display for Value {
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
            Self::Obj(obj) => {
                write!(f, "{obj}")
            }
        }
    }
}
