mod conversions;
pub mod objects;
mod operators;

use std::{
    ptr::NonNull,
};

use crate::{
    value::objects::{FunctionObject, NativeFn, Object, ObjectPointer, ObjectType}, vm::{errors::VMError, VM}
};


#[derive(Debug, Clone, PartialEq)]
/// This stores literal values, you can say copy type or values stored on the stack. String in this enum is not created at runtime, and should only be consumed by compiler to write relevant bytecode
pub enum Literal {
    /// Represents boolean variant which also stores value
    Bool(bool),
    /// Equivalent to `null`
    Nil,
    /// Numbers are represented as `f64`
    Number(f64),
    /// Stores string literals. Should be dropped as soon as bytecode is written. Should not be created at runtime, since it's not getting garbage collected.
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
    /// Creates a `Value` object from the `String`. Since it's created at runtime, it'll have `Obj` variant
    pub fn from_runtime_str(value: String, vm: &mut VM) -> Result<Value, VMError> {
        let obj_pointer = Object::from_str(value, vm)?;
        Ok(Self::Obj(obj_pointer))
    }
    /// Creates a `Value` object from the `FunctionObject`. Since it's created at runtime, it'll have `Obj` variant
    pub fn from_runtime_function(value: FunctionObject, vm: &mut VM) -> Result<Value, VMError> {
        let obj_pointer = Object::from_function_object(value, vm)?;
        Ok(Self::Obj(obj_pointer))
    }
    
    /// Creates a `Value` object from the `FunctionObject`. Since it's created at runtime, it'll have `Obj` variant
    pub fn from_runtime_native(value: NativeFn, vm: &mut VM) -> Result<Value, VMError> {
        let obj_pointer = Object::from_native_object(value, vm)?;
        Ok(Self::Obj(obj_pointer))
    }

    /// Used to generate constant default/Nil value.
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
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Obj(_))
    }

    /// Used to invert the truthy value
    pub fn is_falsey(self) -> bool {
        self.is_nil() || (self.is_bool() && !(Into::<bool>::into(self)))
    }

    /// Destroys the value object, because `self` is moved, and gets inner `f64`
    pub fn to_number(self) -> f64 {
        self.into()
    }

    /// Destroys the value object, because `self` is moved, and gets inner `ObjectPointer`
    pub fn as_object(self) -> ObjectPointer {
        self.into()
    }

    /// Returns the reference to inner `ObjectPointer`.
    pub fn as_object_ref(&self) -> &ObjectPointer {
        match self {
            Self::Obj(op) => op,
            _ => unreachable!(),
        }
    }
    /// Returns the mutable reference to inner `ObjectPointer`.
    pub fn as_object_mut(&mut self) -> &mut Object {
        match self {
            Self::Obj(op) => unsafe { op.as_mut() },
            _ => unreachable!(),
        }
    }

    /// Returns the reference to the function object
    pub fn as_function_ref(&self) -> &FunctionObject {
        match self {
            Self::Obj(obj) => unsafe {
                match &obj.as_ref().ty {
                    ObjectType::Function(f) => f,
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    /// Returns the reference to the function object
    pub fn as_function_mut(&mut self) -> &mut FunctionObject {
        match self {
            Self::Obj(obj) => unsafe {
                match &mut obj.as_mut().ty {
                    ObjectType::Function(f) => f,
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    /// Returns the reference to the native object
    pub fn as_native_ref(&self) -> &NativeFn {
        match self {
            Self::Obj(obj) => unsafe {
                match &obj.as_ref().ty {
                    ObjectType::Native(f) => f,
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    /// Returns the mutable reference to the native object
    pub fn as_native_mut(&mut self) -> &mut NativeFn {
        match self {
            Self::Obj(obj) => unsafe {
                match &mut obj.as_mut().ty {
                    ObjectType::Native(f) => f,
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }

    /// Destroys the value object, because `self` is moved, and gets the inner `NativeFn`
    pub fn as_native_object(self) -> NativeFn {
        self.into()
    }

    /// Destroys the value object, because `self` is moved, and gets the inner `FunctionObject`
    pub fn as_function_object(self) -> FunctionObject {
        self.into()
    }

    /// Destroys the value object, because `self` is moved, and gets the inner `String`
    pub fn as_string(self) -> String {
        self.into()
    }

    /// Checks if the string is of type `Literal`, and is created at compile time
    pub fn is_literal_string(&self) -> bool {
        match self {
            Self::Literal(Literal::String(_)) => true,
            _ => false,
        }
    }

    /// Checks if the string is of type `Obj`, and is created at runtime
    pub fn is_object_string(&self) -> bool {
        unsafe {
            match self {
                Self::Obj(obj) if matches!((obj.as_ref()).ty, ObjectType::String(_)) => true,
                _ => false,
            }
        }
    }

    /// Checks if the string is of type `Obj`, and is created at runtime
    pub fn is_function(&self) -> bool {
        unsafe {
            match self {
                Self::Obj(obj) if matches!((obj.as_ref()).ty, ObjectType::Function(_)) => true,
                _ => false,
            }
        }
    }
    
    /// Checks if the string is of type `Obj`, and is created at runtime
    pub fn is_native(&self) -> bool {
        unsafe {
            match self {
                Self::Obj(obj) if matches!((obj.as_ref()).ty, ObjectType::Native(_)) => true,
                _ => false,
            }
        }
    }

    /// Checks if `Value` is a string
    pub fn is_string(&self) -> bool {
        self.is_object_string() || self.is_literal_string()
    }
}

/// Implements Display trait for nicer output
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(Literal::Nil) => {
                write!(f, "nil")
            }
            Self::Literal(Literal::Bool(b)) => {
                write!(f, "{b}")
            }
            Self::Literal(Literal::Number(n)) => {
                write!(f, "{n}")
            }
            Self::Literal(Literal::String(s)) => {
                let s = s.replace("\\n", "\n");
                write!(f, "{s}")
            }
            Self::Obj(obj) => unsafe { write!(f, "{}", obj.as_ref()) },
        }
    }
}
