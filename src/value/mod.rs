use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
    ptr::NonNull,
};

use crate::{
    chunk::Chunk,
    vm::{VM, VMError},
};

#[derive(Debug, Clone, PartialEq)]
/// Type to store object types and associated data
pub enum ObjectType {
    /// Stores owned pointer to the String allocated on heap
    String(Box<String>),
    Function(Box<FunctionObject>),
}

/// `Display` trait implementation to display `ObjectType`s nicely
impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                // Display string values in double quotes
                write!(f, "{s}")
            }
            Self::Function(fun) => {
                write!(f, "{fun}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionObject {
    pub arity: i32,
    pub chunk: Chunk,
    pub name: Option<String>,
}

impl Display for FunctionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = self.name.as_ref() {
            write!(f, "<fn {n}>")
        } else {
            write!(f, "<script>")
        }
    }
}

impl FunctionObject {
    pub fn new() -> Self {
        Self {
            arity: 0,
            chunk: Self::init_chunk(),
            name: None,
        }
    }

    fn init_chunk() -> Chunk {
        Chunk::new()
    }
}

/// Type to store a raw pointer to `Object` stored on heap. `NonNull` ensures that raw pointer is not null and also is space efficient.
pub type ObjectPointer = NonNull<Object>;

/// Type to store reference to the Object for garbage collection
pub type ObjectNode = Option<ObjectPointer>;

#[derive(Debug, Clone, PartialEq)]
/// Data structure to store the `ObjectType` (which owns the value) and `next` node, for garbage collection
pub struct Object {
    /// Stores the type of the `Object` being created
    ty: ObjectType,
    /// Stores the raw pointer to the next node. If an expression has allocated runtime memory for objects, it's possible that more than one objects are linked. Freeing one object should free other objects too.
    pub next: ObjectNode,
}

impl Object {
    /// Returns the fresh instance of `Object`
    pub fn new(ty: ObjectType) -> Self {
        Self { ty, next: None }
    }

    /// All runtime objects should be created with this method. It's important for garbage collection
    pub fn with_vm(ty: ObjectType, vm: &mut VM) -> Result<ObjectPointer, VMError> {
        // Moves the reference of head of the list to the `objects` variable. `vm.objects` will be `None` after this.
        let objects = vm.objects.take();

        // If `debug_trace_execution` is enabled, show what object has been added on runtime
        // todo! see if we should add another feature for GC
        #[cfg(feature = "debug_trace_execution")]
        {
            println!("-------GC Insert---------");
            println!("{ty}");
            println!("-------------------------");
        }

        // Create an object, `next` pointing to current head of the list
        let obj = Self { ty, next: objects };
        // Allocate `Object` on heap, by using `Box`
        let boxed_obj = Box::new(obj);
        // Convert `Box` pointer into raw pointer, create a NonNull pointer from raw_pointer
        let obj_ptr = NonNull::new(Box::into_raw(boxed_obj)).ok_or(VMError::RuntimeError(
            "Can't convert object into NonNull pointer".to_owned(),
        ))?;

        // Point `vm.objects` to newly added node
        vm.objects = Some(obj_ptr);
        // Return the pointer
        Ok(obj_ptr)
    }

    /// Creates `Object` of type `String` on runtime.
    pub fn from_str(value: String, vm: &mut VM) -> Result<ObjectPointer, VMError> {
        // Create an owned pointer to string, not object it self, and pass to `with_vm` function. This distinction is important because ObjectType::String owns the string value, but this method returns the pointer to the object created.
        Self::with_vm(ObjectType::String(Box::new(value)), vm)
    }

    /// Creates `Object` of type `FunctionObject` at runtime.
    pub fn from_function_object(
        fun_obj: FunctionObject,
        vm: &mut VM,
    ) -> Result<ObjectPointer, VMError> {
        // Create an owned pointer to string, not object it self, and pass to `with_vm` function. This distinction is important because ObjectType::String owns the string value, but this method returns the pointer to the object created.
        Self::with_vm(ObjectType::Function(Box::new(fun_obj)), vm)
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
    pub fn is_obj(&self) -> bool {
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

    /// Destroys the value object, because `self` is moved, and gets the inner `String` created at runtime
    pub fn as_object_string(self) -> String {
        self.into()
    }
    /// Destroys the value object, because `self` is moved, and gets the inner `String` created at runtime
    pub fn as_function_object(self) -> FunctionObject {
        self.into()
    }

    /// Destroys the value object, because `self` is moved, and gets the inner `String` created at compile time
    pub fn as_literal_string(self) -> String {
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

    /// Checks if `Value` is a string
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
            // String is create at runtime, some unsafe code is needed to handle raw pointers.
            // Before calling `.into()`, it should be checked that value is indeed a string.
            Self::Obj(n) => unsafe {
                // Get the raw pointer to the string
                let raw_ptr = n.as_ptr();
                // Convert raw pointer to the owned pointer. It's unsafe operation. It's important to extract value from the `NonNull` pointer.
                // --------- IMPORTANT NOTE ---------
                // This gets the inner value from pointer and moves it to owned pointer. This will invalidate existing pointers, such as stored in `vm.objects`. Moving into owned string will require pointers to be removed manually from the list
                // --------- /IMPORTANT NOTE --------
                let boxed_obj = Box::from_raw(raw_ptr);
                match (boxed_obj).ty {
                    // If Object is of type string, just move the string out of the box
                    ObjectType::String(s) => *s,
                    _ => unreachable!(),
                }
            },
            // If string is Literal, created at compile time, just move out of the enum
            Self::Literal(Literal::String(s)) => s,
            // Can't handle errors at this level, errors are handled on compiler level
            // for detailed output
            _ => unreachable!(),
        }
    }
}
/// Implements `Into` trait to extract `Obj` from `Value::Obj`
impl Into<FunctionObject> for Value {
    fn into(self) -> FunctionObject {
        match self {
            // Function is created at runtime, some unsafe code is needed to handle raw pointers.
            // Before calling `.into()`, it should be checked that value is indeed a `FunctionObject`.
            Self::Obj(n) => unsafe {
                // Get the raw pointer to the `FunctionObject`
                let raw_ptr = n.as_ptr();
                // Convert raw pointer to the owned pointer. It's unsafe operation. It's important to extract value from the `NonNull` pointer.
                // --------- IMPORTANT NOTE ---------
                // This gets the inner value from pointer and moves it to owned pointer. This will invalidate existing pointers, such as stored in `vm.objects`. Moving into owned `FunctionObject` will require pointers to be removed manually from the list
                // --------- /IMPORTANT NOTE --------
                let boxed_obj = Box::from_raw(raw_ptr);
                match (boxed_obj).ty {
                    // If Object is of type `FunctionObject`, just move the `FunctionObject` out of the box
                    ObjectType::Function(fun) => *fun,
                    _ => unreachable!(),
                }
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

/// Implements `From` trait to convert from `Object` to `Value::Obj`
impl From<FunctionObject> for Value {
    fn from(value: FunctionObject) -> Self {
        let object_type = ObjectType::Function(Box::new(value));
        let object = Object::new(object_type);
        // `unwrap()` shouldn't be used here. Alternatively consider using `Option<NonNull<Object>>` in `Value::Obj`
        let pointer = NonNull::new(Box::into_raw(Box::new(object))).unwrap();
        Self::Obj(pointer)
    }
}

/// Overrides '+' operator for custom type `Value`
/// It's like operator overloading in C++
/// Only works if `Value` is of type number
impl Add for Value {
    type Output = self::Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_number() && rhs.is_number() {
            // Convert both operands into numbers.
            let a: f64 = self.into();
            let b: f64 = rhs.into();
            // Convert both numbers into value
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
                write!(f, "{s}")
            }
            Self::Obj(obj) => unsafe { write!(f, "{}", obj.as_ref()) },
        }
    }
}
