use crate::{
    chunk::OpCode,
    compiler::{CompilationContext, CompilerState, errors::CompilerError, types::FunctionType},
    value::FunctionObject,
};

fn compile(code: &str) -> Result<FunctionObject, CompilerError> {
    let mut context = CompilationContext::new(code);
    let function_type = FunctionType::default_script();
    context.push(CompilerState::new(function_type));
    context.compile()
}

#[test]
fn test_var_declaration() {
    let fun_obj = compile("var a;").unwrap();
    let code = fun_obj.chunk.code;
    let expected_value = vec![
        // Bytecode for Value of the expression evaluated. Since the code doesn't have any value, it automatically assigns `OpCode::OpNil`. Expression should be evaluated first, so it comes on the stack first
        OpCode::OpNil as u8,
        // After evaluating expression, check if variable is global or local, by comparing scope_depth. In this case, it's `OpCode::OpSetGlobal`
        OpCode::OpDefineGlobal as u8,
        0,                      // Index/Position of the value of defined variable on the stack
        OpCode::OpNil as u8,    // Since it's a top level function, it always returns `Nil`
        OpCode::OpReturn as u8, // OpCode::OpReturn to stop the virtual machine.
    ];
    assert_eq!(code, expected_value);
}

#[test]
fn test_var_initialization() {
    let fun_obj = compile(r#"var a= 10 + 20;"#).unwrap();
    let code = fun_obj.chunk.code;
    let expected_value = vec![
        OpCode::OpConstant as u8, // Constant OpCode
        1, // Position of constant value in constant pool, 20 has position 1 but will be emitted first
        OpCode::OpConstant as u8, // Constant OpCode
        2, // Position of constant value in constant pool
        OpCode::OpAdd as u8, // Print Opcode, after expression is evaluated.
        OpCode::OpDefineGlobal as u8,
        0,                      // Position of variable name in the constant pool
        OpCode::OpNil as u8,    // Since it's a top level function, it always returns `Nil`
        OpCode::OpReturn as u8, // OpCode::OpReturn to stop the virtual machine.
    ];
    assert_eq!(code, expected_value);
}

#[test]
fn test_print_statement() {
    let fun_obj = compile(r#"print "Hamza";"#).unwrap();
    let code = fun_obj.chunk.code;
    let expected_value = vec![
        OpCode::OpConstant as u8, // Constant OpCode
        0,                        // Position of constant value in constant pool
        OpCode::OpPrint as u8,    // Print Opcode, after expression is evaluated.
        OpCode::OpNil as u8,      // Since it's a top level function, it always returns `Nil`
        OpCode::OpReturn as u8,   // OpCode::OpReturn to stop the virtual machine.
    ];
    assert_eq!(code, expected_value);
}

#[test]
fn test_function_declaration() {
    let fun_obj = compile(
        "
        fun printHello(){
            print \"Hello\";        
        }

        printHello();
    ",
    )
    .unwrap();
    let code = fun_obj.chunk.code;
    let expected_bytecode = vec![
        OpCode::OpConstant as u8,     // Instruction for OpConstant
        1,                            // Position for value on constant pool
        OpCode::OpDefineGlobal as u8, // OpDefineGlobal to define variable (function in this case)
        0,                            // Position of function name in constant pool.
        OpCode::OpGetGlobal as u8,    // byte OpGetGlobal
        2,                            // Variable offset in byte_code.
        OpCode::OpCall as u8,         // OpCall
        0,                            // argument count for call
        OpCode::OpPop as u8,          // OpPop
        OpCode::OpNil as u8,          // OpNil
        OpCode::OpReturn as u8,       // OpReturn
    ];

    assert_eq!(expected_bytecode, code);
}
