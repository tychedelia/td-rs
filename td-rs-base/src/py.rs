use pyo3_ffi::PyGetSetDef;

pub trait PyGetSets {
    const SIZE: usize;
    fn get_get_sets() -> &'static [pyo3_ffi::PyGetSetDef];
}

impl<T> PyGetSets for T {
    const SIZE: usize = 0;

    fn get_get_sets() -> &'static [PyGetSetDef] {
        &[]
    }
}

pub trait PyMethods {
    const SIZE: usize;
    fn get_methods() -> &'static [pyo3_ffi::PyMethodDef];
}

impl<T> PyMethods for T {
    const SIZE: usize = 0;
    fn get_methods() -> &'static [pyo3_ffi::PyMethodDef] {
        &[]
    }
}


pub(crate) unsafe fn py_op_info<T: PyMethods + PyGetSets>(op_info: std::pin::Pin<&mut crate::cxx::OP_CustomOPInfo>) {
    let methods = T::get_methods();
    let m_arr = methods.as_ptr() as *mut autocxx::prelude::c_void;
    let get_sets = T::get_get_sets();
    let gs_arr = get_sets.as_ptr() as *mut autocxx::prelude::c_void;
    crate::cxx::setPyInfo(op_info, m_arr, <T as PyMethods>::SIZE, gs_arr, <T as PyGetSets>::SIZE);
}
