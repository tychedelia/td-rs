use std::ops::Deref;
use std::pin::Pin;

/// Input to an operator, which can be used to get parameters, channels,
/// and other information.
pub struct OperatorInput<'execute> {
    input: Pin<&'execute crate::cxx::ffi::OperatorInput>,
}

impl<'execute> OperatorInput<'execute> {
    /// Create a new operator input.
    pub fn new(input: Pin<&'execute crate::cxx::ffi::OperatorInput>) -> OperatorInput<'execute> {
        Self { input }
    }

    /// Get a float parameter.
    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        self.input.getParDouble(&(name.to_string()), index as i32)
    }

    /// Get an integer parameter.
    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        self.input.getParInt(name, index as i32)
    }

    /// Get a string parameter.
    pub fn get_string(&self, name: &str) -> &str {
        self.input.getParString(name)
    }
}