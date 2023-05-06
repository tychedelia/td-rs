use std::pin::Pin;

pub struct OperatorInput<'execute> {
    input: Pin<&'execute crate::cxx::ffi::OperatorInput>,
}

impl<'execute> OperatorInput<'execute> {
    pub fn new(
        input: Pin<&'execute crate::cxx::ffi::OperatorInput>,
    ) -> OperatorInput<'execute> {
        Self { input }
    }

    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        println!("get_float: {} {}", name, index);
        self.input.getParDouble(name, index as i32)
    }

    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        self.input.getParInt(name, index as i32)
    }

    pub fn get_string(&self, name: &str) -> &str {
        self.input.getParString(name)
    }
}
