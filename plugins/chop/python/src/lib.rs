use pyo3::impl_::pyclass::PyClassImpl;
use pyo3::types::PyDict;
use pyo3::{prelude::*, PyMethodDef};
use pyo3::{AsPyPointer, PyNativeType};
use std::collections::HashMap;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

fn foo() {
    let a = crate::double::DEF;
    println!("{:?}", a.as_method_def());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::foo();
    }
}
