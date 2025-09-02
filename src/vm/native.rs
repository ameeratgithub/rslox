use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    value::{Value, objects::NativeFn},
    vm::{VM, errors::VMError},
};

impl VM {
    pub(super) fn define_native(&mut self, name: &str, function: NativeFn) -> Result<(), VMError> {
        let val = Value::from_runtime_native(function, self)?;
        self.globals.insert(name.to_owned(), val);
        Ok(())
    }
}

pub(super) fn clock_native(_arg_count: u8, _values: Vec<Value>) -> Value {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration.as_secs_f64().into()
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn println(_arg_count: u8, values: Vec<Value>) -> Value {
    if values.is_empty() {
        println!();
    } else {
        println!("{}", values[0]);
    }

    Value::new_nil()
}
