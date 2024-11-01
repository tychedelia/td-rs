use pyo3_ffi::PyGetSetDef;

pub trait PyGetSets {
    fn get_get_sets() -> &'static [pyo3_ffi::PyGetSetDef];
}

impl<T> PyGetSets for T {
    default fn get_get_sets() -> &'static [PyGetSetDef] {
        &[]
    }
}

pub trait PyMethods {
    fn get_methods() -> &'static [pyo3_ffi::PyMethodDef];
}

impl<T> PyMethods for T {
    default fn get_methods() -> &'static [pyo3_ffi::PyMethodDef] {
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
    let gs_len = get_sets.len();
    let gs_arr = get_sets.as_ptr() as *mut autocxx::prelude::c_void;
    crate::cxx::setPyInfo(op_info, m_arr, m_len, gs_arr, gs_len);
}

pub trait FromPyObj {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self;
}

pub trait ToPyObj {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject;
}

pub trait CheckPyObj {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool;
}

#[cfg(windows)]
impl FromPyObj for u32 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsUnsignedLong(obj) as u32
    }
}

impl FromPyObj for usize {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsUnsignedLong(obj) as usize
    }
}

#[cfg(target_os = "macos")]
impl ToPyObj for usize {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromUnsignedLong(self as u64)
    }
}

#[cfg(target_os = "windows")]
impl ToPyObj for usize {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromUnsignedLong(self as u32)
    }
}

impl CheckPyObj for usize {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for isize {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsLong(obj) as isize
    }
}

#[cfg(target_os = "macos")]
impl ToPyObj for isize {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromLong(self as i64)
    }
}

#[cfg(target_os = "windows")]
impl ToPyObj for isize {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromLong(self as i32)
    }
}

impl CheckPyObj for isize {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

#[cfg(target_os = "windows")]
impl FromPyObj for u32 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsUnsignedLong(obj)
    }
}

#[cfg(target_os = "macos")]
impl FromPyObj for u32 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsUnsignedLong(obj) as u32
    }
}

#[cfg(target_os = "windows")]
impl ToPyObj for u32 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromUnsignedLong(self)
    }
}

#[cfg(target_os = "macos")]
impl ToPyObj for u32 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromUnsignedLong(self as u64)
    }
}

impl CheckPyObj for u32 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for i32 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsLong(obj) as i32
    }
}

#[cfg(target_os = "windows")]
impl ToPyObj for i32 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromLong(self)
    }
}

#[cfg(target_os = "macos")]
impl ToPyObj for i32 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromLong(self as i64)
    }
}

impl CheckPyObj for i32 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for u64 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsUnsignedLongLong(obj) as u64
    }
}

impl ToPyObj for u64 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromUnsignedLongLong(self)
    }
}

impl CheckPyObj for u64 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for i64 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyLong_AsLongLong(obj) as i64
    }
}

impl ToPyObj for i64 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyLong_FromLongLong(self)
    }
}

impl CheckPyObj for i64 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for f32 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyFloat_AsDouble(obj) as f32
    }
}

impl ToPyObj for f32 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyFloat_FromDouble(self as f64)
    }
}

impl CheckPyObj for f32 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyFloat_Check(obj) != 0 || pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for f64 {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyFloat_AsDouble(obj)
    }
}

impl ToPyObj for f64 {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        pyo3_ffi::PyFloat_FromDouble(self)
    }
}

impl CheckPyObj for f64 {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyFloat_Check(obj) != 0 || pyo3_ffi::PyLong_Check(obj) != 0
    }
}

impl FromPyObj for bool {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        pyo3_ffi::PyObject_IsTrue(obj) != 0
    }
}

impl ToPyObj for bool {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        if self {
            pyo3_ffi::Py_True()
        } else {
            pyo3_ffi::Py_False()
        }
    }
}

impl CheckPyObj for bool {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyBool_Check(obj) != 0
    }
}

impl FromPyObj for String {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        let s = pyo3_ffi::PyUnicode_AsUTF8(obj);
        let s = std::ffi::CStr::from_ptr(s);
        let s = s.to_str().unwrap();
        s.to_owned()
    }
}

impl ToPyObj for String {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        let s = std::ffi::CString::new(self.as_str()).unwrap();
        pyo3_ffi::PyUnicode_FromString(s.as_ptr())
    }
}

impl CheckPyObj for String {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyUnicode_Check(obj) != 0
    }
}

impl FromPyObj for &str {
    unsafe fn from_py_obj(obj: *mut pyo3_ffi::PyObject) -> Self {
        let s = pyo3_ffi::PyUnicode_AsUTF8(obj);
        let s = std::ffi::CStr::from_ptr(s);
        let s = s.to_str().unwrap();
        s
    }
}

impl ToPyObj for &str {
    unsafe fn to_py_obj(self) -> *mut pyo3_ffi::PyObject {
        let s = std::ffi::CString::new(self.as_bytes()).unwrap();
        pyo3_ffi::PyUnicode_FromString(s.as_ptr())
    }
}

impl CheckPyObj for &str {
    unsafe fn check_py_obj(obj: *mut pyo3_ffi::PyObject) -> bool {
        pyo3_ffi::PyUnicode_Check(obj) != 0
    }
}
