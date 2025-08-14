use std::{
    ops::{Add, Div, Mul, Neg, Not, Sub},
    ptr::NonNull,
};

use crate::vm::{VM, VMError};

#[derive(Debug, Clone, PartialEq)]
/// Type to store object types and associated data
pub enum ObjectType {
    /// Stores owned pointer to the String allocated on heap
    String(Box<String>),
    /// Temporary type, will be removed later
    Other,
}

/// `Display` trait implementation to display `ObjectType`s nicely
impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                // Display string values in double quotes
                write!(f, "\"{s}\"")
            }
            // Temporary type. Will be removed later
            Self::Other => {
                write!(f, "OTHER")
            }
        }
    }
}

/// Type to store a raw pointer to `Object` stored on heap. `NonNull` ensures that raw pointer is not null and also is space efficient.
pub type ObjectPointer = NonNull<Object>;
pub type ObjectNode = Option<ObjectPointer>;

#[derive(Debug, Clone, PartialEq)]
/// Container for the `Object`
pub struct Object {
    /// Stores the type of the `Object` being created
    ty: ObjectType,
    /// Stores the raw pointer to the next node. If an expression has allocated runtime memory for objects, it's possible that more than one objects are linked. Freeing one object should free other objects too.
    pub next: ObjectNode,
}

impl Object {
    pub fn new(ty: ObjectType) -> Self {
        Self { ty, next: None }
    }

    /// All runtime objects should be created with this method. It's important for garbage collection
    pub fn with_vm(ty: ObjectType, vm: &mut VM) -> Result<ObjectPointer, VMError> {
        let objects = vm.objects.take();

        // If `debug_trace_execution` is enabled, show what object has been added on runtime
        #[cfg(feature = "debug_trace_execution")]
        {
            println!("-------GC Insert---------");
            println!("{ty}");
            println!("-------------------------");
        }

        let obj = Self { ty, next: objects };
        let boxed_obj = Box::new(obj);
        // Allocate `Object` on heap, by using `Box`, and convert `Box` pointer into raw pointer
        let obj_ptr = NonNull::new(Box::into_raw(boxed_obj)).ok_or(VMError::RuntimeError(
            "Can't convert object into NonNull pointer".to_owned(),
        ))?;

        vm.objects = Some(obj_ptr);

        Ok(obj_ptr)
    }

    /// Creates `Object` of type `String` on runtime.
    pub fn from_runtime_str(value: String, vm: &mut VM) -> Result<ObjectPointer, VMError> {
        Self::with_vm(ObjectType::String(Box::new(value)), vm)
    }
}

/// Create `Object` from a `String` value
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

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Represents boolean variant which also stores value
    Bool(bool),
    /// Equivalent to `null`
    Nil,
    /// Numbers are represented as `f64`
    Number(f64),
    /// Stores string literals. Should be dropped as soon as bytecode is written
    String(String),
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
    Literal(Literal),
    /// Stores pointer to the object stored on heap
    Obj(NonNull<Object>),
}

impl Value {
    pub fn from_runtime_str(value: String, vm: &mut VM) -> Result<Value, VMError> {
        let obj_pointer = Object::from_runtime_str(value, vm)?;
        Ok(Self::Obj(obj_pointer))
    }

    pub const fn new_nil() -> Value {
        Value::Literal(Literal::Nil)
    }

    /// If value is pf boolean type, returns true
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Literal(Literal::Bool(_)))
    }

    /// If value is nil, returns true
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Literal(Literal::Nil))
    }

    /// Returns true if value is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Literal(Literal::Number(_)))
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
    pub fn as_object(self) -> ObjectPointer {
        self.into()
    }

    /// Just convert `Value` instance to `Object`
    pub fn as_object_ref(&self) -> &ObjectPointer {
        match self {
            Self::Obj(op) => op,
            _ => unreachable!(),
        }
    }

    pub fn as_object_string(self) -> String {
        self.into()
    }

    pub fn as_literal_string(self) -> String {
        self.into()
    }

    pub fn is_literal_string(&self) -> bool {
        match self {
            Self::Literal(Literal::String(_)) => true,
            _ => false,
        }
    }

    pub fn is_object_string(&self) -> bool {
        unsafe {
            match self {
                Self::Obj(obj) if matches!((obj.as_ref()).ty, ObjectType::String(_)) => true,
                _ => false,
            }
        }
    }
    pub fn is_string(&self) -> bool {
        self.is_object_string() || self.is_literal_string()
    }
}

/// Implements `Into` trait to extract `bool` from `Value::Bool`
impl Into<bool> for Value {
    fn into(self) -> bool {
        match self {
            Self::Literal(Literal::Bool(b)) => b,
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
            Self::Literal(Literal::Number(n)) => n,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}

/// Implements `Into` trait to extract `Obj` from `Value::Obj`
impl Into<ObjectPointer> for Value {
    fn into(self) -> ObjectPointer {
        match self {
            Self::Obj(n) => n,
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
            Self::Obj(n) => unsafe {
                let raw_ptr = n.as_ptr();
                let boxed_obj = Box::from_raw(raw_ptr);
                match (boxed_obj).ty {
                    ObjectType::String(s) => *s,
                    _ => unreachable!(),
                }
            },
            Self::Literal(Literal::String(s)) => s,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}

/// Implements `From` trait to convert from `bool` to `Value::Bool`
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Literal(Literal::Bool(value))
    }
}

/// Implements `From` trait to convert from `f64` to `Value::Number`
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Literal(Literal::Number(value))
    }
}


/// Implements `From` trait to convert from `Object` to `Value::Obj`
impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Literal(Literal::String(value))
    }
}

/// Overrides '+' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Add for Value {
    type Output = self::Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a + b).into();
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
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a - b).into();
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
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a * b).into();
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
        if self.is_number() && rhs.is_number() {
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            return (a / b).into();
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
        if self.is_number() {
            let a: f64 = self.into();
            return (-a).into();
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
        if self.is_bool() {
            let b: bool = self.into();
            return (!b).into();
        }

        // This code shouldn't be reached
        unreachable!();
    }
}

/// Implements Display trait for nicer output
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(Literal::Nil) => {
                write!(f, "Nil")
            }
            Self::Literal(Literal::Bool(b)) => {
                write!(f, "{b}")
            }
            Self::Literal(Literal::Number(n)) => {
                write!(f, "{n}")
            }
            Self::Literal(Literal::String(s)) => {
                write!(f, "\"{s}\"")
            }
            Self::Obj(obj) => unsafe { write!(f, "{}", obj.as_ref()) },
        }
    }
}
