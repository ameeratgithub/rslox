# Examples

If you just quickly want to see the output of the VM, just run one of the examples given by passing relevant arguments.

## `basic_arithmetic.rs`

It has code to directly write byte code to a chunk. The virtual machine then executes those instruction. Order of operation does matter.

To run this example, run following command in your terminal:
```bash
cargo run --example basic_arithmetic --features debug_trace_execution
```

## `cli.rs` 

It has boilerplate to test your *Lox* code. To start a REPL, execute following command in your terminal

```bash
cargo run --example cli --features debug_trace_execution
```

If you want to run custom *Lox* written in a file, like in `lox/*`, you can execute following command

```bash
cargo run --example cli --features debug_trace_execution -- --file="[file_path]"
```

where [file_path] would be path of your *Lox* file. For existing lox examples, you can do something like this

```bash
cargo run --example cli --features debug_trace_execution -- --file="lox/expression.lox"
```