use std::cmp::Ordering;

use crate::{
    constants::FRAMES_MAX,
    value::Value,
    vm::{VM, VMError, call_frame::CallFrame},
};

impl VM {
    pub(super) fn op_return(&mut self) -> bool {
        let result = self.pop().unwrap();
        match self.frames.len().cmp(&1) {
            Ordering::Equal => {
                self.pop();
                // End of bytecode
                return true;
            }
            Ordering::Greater => {
                self.pop();
                self.pop();
            }
            Ordering::Less => {}
        }

        self.push(result);
        self.frames.pop();
        // It's just end of a called function, not end of bytecode.
        false
    }

    pub(super) fn op_call(&mut self) -> Result<(), VMError> {
        let arg_count = self.current_frame().read_byte();
        let callee_index = self.stack.len() - (arg_count as usize) - 1;
        let callee = self.stack[callee_index].clone();
        self.call_value(callee, arg_count)
    }

    fn call_value(&mut self, callee: Value, arg_count: u8) -> Result<(), VMError> {
        if callee.is_function() {
            return self.call(callee, arg_count);
        } else if callee.is_native() {
            let native = callee.as_native_ref();

            let mut values = vec![];
            for _ in 0..arg_count {
                values.push(self.pop().unwrap());
            }
            self.pop();

            let result = native(arg_count, values);
            self.push(result);

            return Ok(());
        }

        Err(self.construct_runtime_error(format_args!("Can only call functions and classes")))
    }

    ///
    ///  # Errors
    ///
    /// Returns a `VM` error if there's a problem creating stack frame for function
    pub fn call(&mut self, function: Value, arg_count: u8) -> Result<(), VMError> {
        let arity = function.as_function_ref().arity;

        if arg_count != arity {
            let error = self.construct_runtime_error(format_args!(
                "Expected {arity} arguments but got {arg_count}."
            ));
            return Err(error);
        }

        if self.frames.len() == FRAMES_MAX {
            let error = self.construct_runtime_error(format_args!("Stack overflow."));
            return Err(error);
        }

        let starting_index = self.stack.len() - (arg_count as usize);
        let frame = CallFrame::new(function, 0, starting_index);
        self.frames.push(frame);
        Ok(())
    }
}
