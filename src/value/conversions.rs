use std::ptr::NonNull;

use crate::value::{FunctionObject, Literal, Object, ObjectPointer, ObjectType, Value};

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
                    ObjectType::Function(f) => format!("{}", f),
                }
            },
            _ => format!("{}", self),
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
