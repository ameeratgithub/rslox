use std::{fmt::Display, ptr::NonNull};

use crate::{
    chunk::Chunk,
    vm::{VM, errors::VMError},
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
                let s = s.replace("\\n", "\n");
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
    pub(super) ty: ObjectType,
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
        let obj_ptr = NonNull::new(Box::into_raw(boxed_obj)).ok_or_else(|| {
            vm.construct_runtime_error(format_args!("Can't convert object into NonNull pointer."))
        })?;

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
