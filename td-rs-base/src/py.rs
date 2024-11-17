use pyo3::ffi::PyGetSetDef;
use crate::Op;

pub trait PyOp : Op {

}

pub trait PyGetSets {
    fn get_get_sets() -> &'static [pyo3::ffi::PyGetSetDef];
}

impl<T> PyGetSets for T {
    default fn get_get_sets() -> &'static [PyGetSetDef] {
        &[]
    }
}

pub trait PyMethods {
    fn get_methods() -> &'static [pyo3::ffi::PyMethodDef];
}

impl<T> PyMethods for T {
    default fn get_methods() -> &'static [pyo3::ffi::PyMethodDef] {
        &[]
    }
}

pub(crate) unsafe fn py_op_info<T: PyMethods + PyGetSets>(
    op_info: std::pin::Pin<&mut crate::cxx::OP_CustomOPInfo>,
) {
    let methods = T::get_methods();
    let m_len = methods.len();
    let m_arr = methods.as_ptr() as *mut autocxx::prelude::c_void;
    let get_sets = T::get_get_sets();
    println!("get_sets: {:?}", get_sets);
    let gs_len = get_sets.len();
    let gs_arr = get_sets.as_ptr() as *mut autocxx::prelude::c_void;
    crate::cxx::setPyInfo(op_info, m_arr, m_len, gs_arr, gs_len);
}
