use crate::{
    compiler::{CompilationContext, CompilerState, errors::CompilerError, types::FunctionType},
    value::FunctionObject,
    vm::VMError,
};

fn get_context(code: &str) -> Result<FunctionObject, CompilerError> {
    let mut context = CompilationContext::new(code);
    let function_type = FunctionType::Script(Box::new(FunctionObject::new()));
    context.push(CompilerState::new(function_type));
    context.compile()
}

// #[test]
fn test_var_declaration() {
    let fun_obj = get_context("var a;").unwrap();
    let code = fun_obj.chunk.code;
    let expected_value = vec![
        // Bytecode for Value of the expression evaluated. Since the code doesn't have any value, it automatically assigns `OpCode::OpNil`. Expression should be evaluated first, so it comes on the stack first
        7,
        // After evaluating expression, check if variable is global or local, by comparing scope_depth. In this case, it's `OpCode::OpSetGlobal`
        16, 0, // Position of value of Global variable in the stack
        7, // Since it's a top level function, it always returns `Nil`
        0, // OpCode::OpReturn to stop the virtual machine.
    ];
    assert_eq!(code, expected_value);
}

#[test]
fn test_function_declaration() {
    let fun_obj = get_context(
        "
        fun printHello(){
            print \"Hello\";        
        }

        printHello();
    ",
    )
    .unwrap();
    let code = fun_obj.chunk.code;
    let constants = fun_obj.chunk.constants;
    let expected_bytecode = vec![
        1,  // Instruction for OpConstant
        1,  // Position for value on constant pool
        16, // OpDefineGlobal to define variable (function in this case)
        0,  // Position of function name in constant pool.
        17, // byte OpGetGlobal
        2,  // Variable offset in byte_code.
        24, // OpCall
        0,  // argument count for call
        15, // OpPop
        7,  // OpNil
        0,  // OpReturn
    ];

    assert_eq!(expected_bytecode, code);
}
