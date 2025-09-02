use std::{collections::HashSet, ptr::NonNull};

use crate::{
    value::{
        Value,
        objects::{Object, ObjectPointer},
    },
    vm::VM,
};

impl VM {
    pub fn reset_vm(&mut self) {
        #[cfg(feature = "debug_trace_execution")]
        self.display_garbage_items();
        // Remove items from garbage collection
        self.free_objects();
        // Reset stack to its initial state
        self.reset_stack();
    }

    /// Empties the stack and resets the top to '0'
    pub fn reset_stack(&mut self) {
        let mut hash_set = HashSet::new();
        while let Some(value) = self.pop() {
            self.free_stack_object_memory(value, &mut hash_set);
        }
        self.frames = vec![];
    }

    /// Responsible for freeing the memory allocated by runtime objects, such as string
    pub fn free_objects(&mut self) {
        // Iterate over the list of objects
        while let Some(obj) = self.objects {
            // Unsafe is required to dereference the raw pointer
            unsafe {
                // Assign `next` node to `self.objects`
                self.objects = (*obj.as_ptr()).next;
                // `Box` will automatically free the memory
                // Only free after pointing `self.objects` to `next` of current object
                // Otherwise `self.objects` will point to freed memory
                let _ = Box::from_raw(obj.as_ptr());
            }
        }
    }

    /// Frees object memory behind raw pointers, such as a string or a function
    pub fn free_stack_object_memory(
        &mut self,
        value: Value,
        hash_set: &mut HashSet<ObjectPointer>,
    ) {
        if value.is_object() {
            let object = value.as_object();
            if hash_set.contains(&object) {
                return;
            }

            unsafe {
                hash_set.insert(object);
                let _ = Box::from_raw(object.as_ptr());
            }
        }
    }

    /// This method iterates over linked list and remove a node if pointer matches. Useful method when extracting a value from a raw pointer and that raw pointer needs to be dropped.
    pub fn remove_object_pointer(&mut self, other: &NonNull<Object>) {
        // Tracks current node, starting from head
        let mut current = self.objects;
        // Tracks previous node
        let mut prev: Option<NonNull<Object>> = None;
        // Start iterating the list, starting from head
        while let Some(node) = current {
            if node.eq(other) {
                unsafe {
                    // Get the next pointer of the current node
                    let next = (*node.as_ptr()).next;
                    if let Some(mut prev_node) = prev {
                        // It isn't first node, set `prev.next` to `current.next`. It's like creating a link between nodes and removing itself from the middle
                        prev_node.as_mut().next = next;
                    } else {
                        // It's the first node, remove first node by setting itself to next
                        self.objects = next;
                    }
                }
                // Node removed, return now.
                return;
            }
            // Match not found
            unsafe {
                // Set `prev` to `current`
                prev = current;
                // Move `current` to `next` node
                current = (*node.as_ptr()).next;
            }
        }
    }
}
