use crate::{
    chunk::OpCode,
    value::Value,
    vm::{VM, VMError},
};

impl VM {
    /// This function concatenate strings and manage memory at runtime while doing so. If there are two literal strings in bytecode, concatenation will allocate memory for result, at runtime, and that value should be garbage collected
    fn concatenate_strings(
        &mut self,
        left_operand: Value,
        right_operand: Value,
    ) -> Result<(), VMError> {
        // Check if left_operand is heap allocated string
        let left = if left_operand.is_object() {
            // Get reference to the `ObjectPointer` of `left_operand`
            let left_pointer = left_operand.as_object_ref();
            // Remove that pointer from linked list, because `Value` is going to be extracted
            self.remove_object_pointer(left_pointer);
            // Extract string from the pointer
            left_operand.as_string()
        } else {
            // It's not heap allocated string, so just extract the value
            left_operand.as_string()
        };

        // Check if right_operand is heap allocated string
        let right = if right_operand.is_object() {
            // Get reference to the `ObjectPointer` of `right_operand`
            let right_pointer = right_operand.as_object_ref();
            // Remove that pointer from linked list, because `Value` is going to be extracted
            self.remove_object_pointer(right_pointer);
            // Extract string from the pointer
            right_operand.as_string()
        } else {
            // It's not heap allocated string, so just extract the value
            right_operand.as_string()
        };

        // Because it's a runtime operation, being executed by vm, it needs to create a value
        // by using special functions. This is important for garbage collection.
        let value = Value::from_runtime_str(left + &right, self)
            .map_err(|err| self.construct_runtime_error(format_args!("{err}")))?;
        self.push(value);
        // Return because our work here is done.
        Ok(())
    }

    // Performs the binary operation based on `opcode`.
    // `binary_op` should only be called when `opcode` supports binary operation.
    pub(super) fn binary_op(&mut self, opcode: OpCode) -> Result<(), VMError> {
        // We're reading from left to right. So left operand got pushed first, then the right
        // operand got pushed. Let's say we're evaluating following expression
        // `2 - 1`
        // 1. 2 got pushed -> [2]
        // 2. 1 got pushed -> [2,1]
        // 3. 1 is right operand, and will be popped first, because it's on top
        // 4. 2 is left operand, and it will be popped second.
        // 5. so the correct operation will be left_operand - right_operand
        let right_operand = self.pop().ok_or_else(|| {
            // If value isn't on stack, throw an error
            let err = format_args!("Expected value on stack");
            self.construct_runtime_error(err)
        })?;

        let left_operand = self
            .pop()
            .ok_or_else(|| {
                // If value isn't on stack, throw an error
                let err = format_args!("Expected value on stack");
                self.construct_runtime_error(err)
            })
            // This will get executed if value is on stack
            .and_then(|val| {
                let operands_are_numbers = right_operand.is_number() && val.is_number();
                let one_operand_is_string = right_operand.is_string() || val.is_string();
                // We're only interested if both operands are numbers or both are strings
                if operands_are_numbers || (one_operand_is_string && opcode == OpCode::OpAdd) {
                    Ok(val)
                } else {
                    // Invalid operation on operands, return error
                    let err = format_args!("Invalid operation on these operands.");
                    Err(self.construct_runtime_error(err))
                }
            })?;

        // Concatinate if both operands are strings
        if right_operand.is_string() || left_operand.is_string() {
            return self.concatenate_strings(left_operand, right_operand);
        }

        // Match the opcode and perform the relevant operation
        let result = match opcode {
            // Works because `Add` trait is implemented
            OpCode::OpAdd => left_operand + right_operand,
            // Works because `Sub` trait is implemented
            OpCode::OpSubtract => left_operand - right_operand,
            // Works because `Mul` trait is implemented
            OpCode::OpMultiply => left_operand * right_operand,
            // Works because `Div` trait is implemented
            OpCode::OpDivide => left_operand / right_operand,
            // Checks if left > right
            OpCode::OpGreater => {
                // We've checked that both operands are numbers, so we can safely
                // convert them for comparison
                let res = left_operand.to_number() > right_operand.to_number();
                res.into()
            }
            // Checks if left < right
            OpCode::OpLess => {
                // We've checked that both operands are numbers, so we can safely
                // convert them for comparison
                let res = left_operand.to_number() < right_operand.to_number();
                res.into()
            }
            // This arm should never be matched.
            _ => unreachable!(),
        };

        // push the calculated result back on stack
        self.push(result);
        Ok(())
    }

    pub(super) fn op_negate(&mut self) -> Result<(), VMError> {
        let value = self.pop().ok_or_else(||
                            // Return error if value isn't on stack
                            self.construct_runtime_error(format_args!("Expected an operand.")))?;

        // Operand should be a number
        if value.is_number() {
            // This should work because we've implemented `Neg` trait
            self.push(-value);
        } else {
            // Return error if operand isn't a number
            return Err(self.construct_runtime_error(format_args!("Operand must be a number.")));
        }
        Ok(())
    }

    pub(super) fn op_not(&mut self) -> Result<(), VMError> {
        let value = self
            .pop()
            .ok_or_else(|| {
                // If stack is empty, return error
                let err_message = format_args!("Expected value on stack");
                self.construct_runtime_error(err_message)
            })
            .and_then(|val| {
                // Value should be bool or nil
                if val.is_bool() || val.is_nil() {
                    Ok(val)
                } else {
                    // Since we can't negate other types, return error
                    let err_message = format_args!("Operand of ! operator should be a bool or nil");
                    Err(self.construct_runtime_error(err_message))
                }
            })?;

        // This negates original value and pushes onto the stack
        self.push(Value::from(value.is_falsey()));

        Ok(())
    }

    pub(super) fn op_equal(&mut self) -> Result<(), VMError> {
        let a = self.pop().ok_or_else(|| {
            // Return error if stack is empty
            let arguments = format_args!("Expected value on stack");
            self.construct_runtime_error(arguments)
        })?;
        let b = self.pop().ok_or_else(|| {
            // Return error if stack is empty
            let arguments = format_args!("Expected value on stack");
            self.construct_runtime_error(arguments)
        })?;
        // This is possible because of PartialEq trait implementation
        self.push((a == b).into());
        Ok(())
    }
}
