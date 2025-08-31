use crate::vm::VM;

impl VM {
    pub(super) fn debug(&mut self) {
        // This blocks executes only when this debug tracing feature is enabled.
        #[cfg(feature = "debug_trace_execution")]
        {
            use crate::debug::Debug;
            print!("          ");
            for value in &self.stack {
                print!("[ ");
                print!("{}", value);
                print!(" ]");
            }
            println!("");
            let offset = self.current_frame().ip_offset;
            Debug::dissassemble_instruction(
                &self.current_frame().function.as_function_ref().chunk,
                offset,
            );
        }
    }

    /// Show items in garbadge collection
    #[cfg(feature = "debug_trace_execution")]
    pub fn display_garbage_items(&mut self) {
        println!("====== Garbage Collection Items ======");
        if self.objects.is_some() {
            // Temporary variable to hold objects
            let mut head = self.objects;
            // Execute till the end of the list
            while let Some(obj) = head {
                // Required to access to dereference the raw pointer
                unsafe {
                    // Dereference the object from raw pointer
                    print!("{}", (*obj.as_ptr()));
                    // Assign next object to current list
                    head = (*obj.as_ptr()).next;
                }

                // There is a next object, show arrow to look like it's pointing to next element
                if head.is_some() {
                    print!(" -> ")
                } else {
                    // Break the line
                    println!()
                }
            }
        } else {
            println!("-> No Item Found")
        }

        println!("======================================");
    }
}
