use crate::vm::{VM, VMError};

impl VM {
    pub(super) fn op_get_local(&mut self) {
        let slot = self.current_frame().read_byte();
        let index = self.current_frame().starting_offset + slot as usize;
        let val = self.stack[index].clone();
        self.push(val);
    }
    pub(super) fn op_set_local(&mut self) {
        let slot = self.current_frame().read_byte();
        let val = self.stack[self.stack.len() - 1].clone();
        let index = self.current_frame().starting_offset + slot as usize;
        self.replace_or_push(val, index);
    }
    /// Define a global variable and insert into `HashMap`
    pub(super) fn op_define_global(&mut self) -> Result<(), VMError> {
        // Read the variable name from bytecode and convert it to literal string
        let name = self.current_frame().read_constant().as_string();
        // If variable is not initilized, default value stored on stack should be `Nil`. In both cases, we're expecting value on the stack.
        let value= self.pop().ok_or_else(||
                            // Return error if value on stack is not found
                            self.construct_runtime_error(format_args!("Expected value on the stack")))?;
        // Insert variable's name and value into `HashMap`
        self.globals.insert(name, value);
        Ok(())
    }

    /// Gets the value of variable and pushes onto the stack
    pub(super) fn op_get_global(&mut self) -> Result<(), VMError> {
        // Read the variable name from bytecode and convert it to literal string
        let name = self.current_frame().read_constant().as_string();
        // Get the global variable from `HashMap`
        let value = self.globals.get(&name).cloned().ok_or_else(|| {
            // Variable doesn't exist. Return an error.
            self.construct_runtime_error(format_args!("Undefined variable '{name}'"))
        })?;
        // Variable exists, push value on the stack for later use.
        self.push(value);
        Ok(())
    }

    /// Sets value to already declared global variable
    pub(super) fn op_set_global(&mut self) -> Result<(), VMError> {
        // Read the variable name from bytecode and convert it to literal string
        let name = self.current_frame().read_constant().as_string();
        // Check for underflow. If `stack_top` is less than zero after subtraction, return error
        let value_index =
            self.stack.len().checked_sub(1).ok_or_else(|| {
                self.construct_runtime_error(format_args!("Expected value on stack"))
            })?;
        // Clone value from the stack. We just want to store it in HashMap, so no need to pop or replace value.
        let value = self.stack[value_index].clone();
        // Check whether variable is defined or not
        if !self.globals.contains_key(&name) {
            // Variable has not been defined, return error.
            return Err(self.construct_runtime_error(format_args!("Undefined variable '{}'", name)));
        }
        // Variable has been defined. Update it's value
        self.globals.insert(name, value);
        
        Ok(())
    }
}
