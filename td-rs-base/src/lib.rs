#![feature(associated_type_defaults)]
#![feature(min_specialization)]

use std::ffi;
use std::fmt::Formatter;
use std::ops::Index;
use std::pin::Pin;
use std::sync::Mutex;

pub use param::*;
#[cfg(feature = "python")]
pub use py::*;

#[cfg(feature = "tokio")]
pub static RUNTIME: std::sync::LazyLock<tokio_core::runtime::Runtime> = std::sync::LazyLock::new(|| {
    tokio_core::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime")
});

pub mod chop;
pub mod cxx;
pub mod dat;
pub mod param;
#[cfg(feature = "python")]
pub mod py;
pub mod sop;
pub mod top;

static INFO_STR: Mutex<String> = Mutex::new(String::new());
static ERROR_STR: Mutex<String> = Mutex::new(String::new());
static WARNING_STR: Mutex<String> = Mutex::new(String::new());

/// Metadata describing the operator plugin.
pub trait OpInfo {
    /// The type of the operator.
    const OPERATOR_TYPE: &'static str = "";
    /// The label of the operator.
    const OPERATOR_LABEL: &'static str = "";
    /// The icon of the operator.
    const OPERATOR_ICON: &'static str = "";
    /// The minimum number of inputs the operator accepts.
    const MIN_INPUTS: usize = 0;
    /// The maximum number of inputs the operator accepts.
    const MAX_INPUTS: usize = 0;
    /// The author name of the operator.
    const AUTHOR_NAME: &'static str = "";
    /// The author email of the operator.
    const AUTHOR_EMAIL: &'static str = "";
    /// The major version of the operator.
    const MAJOR_VERSION: i32 = 0;
    /// The minor version of the operator.
    const MINOR_VERSION: i32 = 0;
    /// Whether to cook on start.
    const COOK_ON_START: bool = false;
    /// Python callbacks DAT.
    const PYTHON_CALLBACKS_DAT: &'static str = "";
}

pub trait InfoChop {
    fn size(&self) -> usize;

    fn channel(&self, index: usize) -> (String, f32);
}

pub trait InfoDat {
    fn size(&self) -> (u32, u32);

    fn entry(&self, index: usize, entry_index: usize) -> String;
}

pub trait OpNew {
    fn new(info: NodeInfo) -> Self;
}

/// Functionality for all operator plugin types.
/// This can commonly be left as the default implementation for most plugins.
pub trait Op {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn info_dat(&self) -> Option<Box<&dyn InfoDat>> {
        None
    }

    fn info_chop(&self) -> Option<Box<&dyn InfoChop>> {
        None
    }

    fn set_info(&mut self, info: &str) {
        INFO_STR.lock().unwrap().replace_range(.., info);
    }

    fn info(&self) -> String {
        INFO_STR.lock().unwrap().clone()
    }

    fn set_error(&mut self, error: &str) {
        ERROR_STR.lock().unwrap().replace_range(.., error);
    }

    fn error(&self) -> String {
        ERROR_STR.lock().unwrap().clone()
    }

    fn set_warning(&mut self, warning: &str) {
        WARNING_STR.lock().unwrap().replace_range(.., warning);
    }

    fn warning(&self) -> String {
        WARNING_STR.lock().unwrap().clone()
    }

    fn pulse_pressed(&mut self, _name: &str) {}
}

pub struct NodeInfo {
    info: &'static cxx::OP_NodeInfo,
}

impl NodeInfo {
    pub fn new(info: &'static cxx::OP_NodeInfo) -> Self {
        Self { info }
    }

    pub fn context(&self) -> Context {
        Context {
            context: self.info.context,
        }
    }
}

pub struct Context {
    #[allow(dead_code)]
    context: *mut cxx::OP_Context,
}

impl Context {
    #[cfg(feature = "python")]
    pub fn create_arguments_tuple(&self, nargs: usize) -> *mut pyo3_ffi::PyObject {
        let obj = unsafe {
            let mut ctx = Pin::new_unchecked(&mut *self.context);
            let tuple = ctx
                .createArgumentsTuple(autocxx::c_int(nargs as i32), std::ptr::null_mut());
            tuple
        };
        obj as *mut pyo3_ffi::PyObject
    }

    #[cfg(feature = "python")]
    pub fn call_python_callback(
        &self,
        callback: &str,
        args: *mut pyo3_ffi::PyObject,
        kw: *mut pyo3_ffi::PyObject,
    ) -> *mut pyo3_ffi::PyObject {
        let callback = ffi::CString::new(callback).unwrap();
        let obj = unsafe {
            let mut ctx = Pin::new_unchecked(&mut *self.context);
            let res = ctx.callPythonCallback(
                callback.as_ptr(),
                args as *mut cxx::_object,
                kw as *mut cxx::_object,
                std::ptr::null_mut(),
            );
            res
        };
        obj as *mut pyo3_ffi::PyObject
    }
}

impl std::fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeInfo")
    }
}

/// Input to an operator, which can be used to get parameters, channels,
/// and other information.
pub struct OperatorInputs<'cook, Op> {
    pub inputs: &'cook cxx::OP_Inputs,
    _marker: std::marker::PhantomData<Op>,
}

impl<T> std::fmt::Debug for OperatorInputs<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OperatorInputs")
    }
}

impl<'cook, Op> OperatorInputs<'cook, Op>
where
    Self: GetInput<'cook, Op>,
{
    /// Create a new operator input. This is only called by the operator.
    pub fn new(inputs: &'cook crate::cxx::OP_Inputs) -> OperatorInputs<'cook, Op> {
        Self {
            inputs,
            _marker: Default::default(),
        }
    }

    /// Get the parameters for this operator.
    pub fn params(&self) -> ParamInputs {
        ParamInputs::new(self.inputs)
    }

    /// Get an input channel.
    pub fn input(&self, index: usize) -> Option<&<Self as GetInput<'cook, Op>>::Input>
    where
        OperatorInputs<'cook, Op>: GetInput<'cook, Op>,
    {
        GetInput::input(self, index)
    }

    /// Get the number of input channels.
    pub fn num_inputs(&self) -> usize
    where
        OperatorInputs<'cook, Op>: GetInput<'cook, Op>,
    {
        GetInput::num_inputs(self)
    }
}

/// Parameter inputs to an operator.
pub struct ParamInputs<'cook> {
    inputs: &'cook crate::cxx::OP_Inputs,
}

impl<'cook> ParamInputs<'cook> {
    /// Create a new operator input. This is only called by the operator.
    pub fn new(inputs: &'cook crate::cxx::OP_Inputs) -> ParamInputs<'cook> {
        Self { inputs }
    }

    /// Get a float parameter.
    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        unsafe {
            self.inputs
                .getParDouble(ffi::CString::new(name).unwrap().into_raw(), index as i32)
        }
    }

    /// Get an integer parameter.
    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        unsafe {
            self.inputs
                .getParInt(ffi::CString::new(name).unwrap().into_raw(), index as i32)
        }
    }

    /// Get a string parameter.
    pub fn get_string(&self, name: &str) -> &str {
        unsafe {
            let res = self
                .inputs
                .getParString(ffi::CString::new(name).unwrap().into_raw());
            ffi::CStr::from_ptr(res).to_str().unwrap()
        }
    }

    /// Get a toggle parameter.
    pub fn get_toggle(&self, name: &str) -> bool {
        unsafe {
            self.inputs
                .getParInt(ffi::CString::new(name).unwrap().into_raw(), 0)
                != 0
        }
    }

    /// Enable or disable a parameter.
    pub fn enable_param(&self, name: &str, enable: bool) {
        unsafe {
            self.inputs
                .enablePar(ffi::CString::new(name).unwrap().into_raw(), enable);
        }
    }

    /// Get a chop parameter.
    fn get_chop(&self, name: &str) -> ChopParam {
        unsafe {
            let chop = self
                .inputs
                .getParCHOP(ffi::CString::new(name).unwrap().into_raw());
            if chop.is_null() {
                ChopParam { input: None }
            } else {
                ChopParam { input: Some(chop) }
            }
        }
    }

    fn get_sop(&self, name: &str) -> SopParam {
        unsafe {
            let sop = self
                .inputs
                .getParSOP(ffi::CString::new(name).unwrap().into_raw());
            if sop.is_null() {
                SopParam { input: None }
            } else {
                SopParam { input: Some(sop) }
            }
        }
    }

    fn get_top(&self, name: &str) -> TopParam {
        unsafe {
            let top = self
                .inputs
                .getParTOP(ffi::CString::new(name).unwrap().into_raw());
            if top.is_null() {
                TopParam { input: None }
            } else {
                TopParam { input: Some(top) }
            }
        }
    }

    fn get_dat(&self, name: &str) -> DatParam {
        unsafe {
            let dat = self
                .inputs
                .getParDAT(ffi::CString::new(name).unwrap().into_raw());
            if dat.is_null() {
                DatParam { input: None }
            } else {
                DatParam { input: Some(dat) }
            }
        }
    }

    #[cfg(feature = "python")]
    fn get_python(&self, name: &str) -> *mut pyo3_ffi::PyObject {
        unsafe {
            let python = self
                .inputs
                .getParPython(ffi::CString::new(name).unwrap().into_raw());
            if python.is_null() {
                std::ptr::null_mut()
            } else {
                python as *mut pyo3_ffi::PyObject
            }
        }
    }

    fn get_double_arr<const N: usize>(&self, name: &str) -> [f64; N] {
        assert!(N > 1 && N <= 4);
        unsafe {
            let mut arr = [0.0; N];
            let name = ffi::CString::new(name).unwrap().into_raw();
            match N {
                2 => {
                    let mut a = 0.0;
                    let mut b = 0.0;
                    self.inputs
                        .getParDouble2(name, Pin::new(&mut a), Pin::new(&mut b));
                    arr[0] = a;
                    arr[1] = b;
                }
                3 => {
                    let mut a = 0.0;
                    let mut b = 0.0;
                    let mut c = 0.0;
                    self.inputs.getParDouble3(
                        name,
                        Pin::new(&mut a),
                        Pin::new(&mut b),
                        Pin::new(&mut c),
                    );
                    arr[0] = a;
                    arr[1] = b;
                    arr[2] = c;
                }
                4 => {
                    let mut a = 0.0;
                    let mut b = 0.0;
                    let mut c = 0.0;
                    let mut d = 0.0;
                    self.inputs.getParDouble4(
                        name,
                        Pin::new(&mut a),
                        Pin::new(&mut b),
                        Pin::new(&mut c),
                        Pin::new(&mut d),
                    );
                    arr[0] = a;
                    arr[1] = b;
                    arr[2] = c;
                    arr[3] = d;
                }
                _ => {}
            };

            arr
        }
    }

    fn get_int_arr<const N: usize>(&self, name: &str) -> [i32; N] {
        assert!(N > 1 && N <= 4);
        unsafe {
            let mut arr = [0; N];
            let name = ffi::CString::new(name).unwrap().into_raw();
            match N {
                2 => {
                    let mut a = 0;
                    let mut b = 0;
                    self.inputs
                        .getParInt2(name, Pin::new(&mut a), Pin::new(&mut b));
                    arr[0] = a;
                    arr[1] = b;
                }
                3 => {
                    let mut a = 0;
                    let mut b = 0;
                    let mut c = 0;
                    self.inputs.getParInt3(
                        name,
                        Pin::new(&mut a),
                        Pin::new(&mut b),
                        Pin::new(&mut c),
                    );
                    arr[0] = a;
                    arr[1] = b;
                    arr[2] = c;
                }
                4 => {
                    let mut a = 0;
                    let mut b = 0;
                    let mut c = 0;
                    let mut d = 0;
                    self.inputs.getParInt4(
                        name,
                        Pin::new(&mut a),
                        Pin::new(&mut b),
                        Pin::new(&mut c),
                        Pin::new(&mut d),
                    );
                    arr[0] = a;
                    arr[1] = b;
                    arr[2] = c;
                    arr[3] = d;
                }
                _ => {}
            };
            arr
        }
    }
}

/// Get an input to an operator.
pub trait GetInput<'cook, Op>: Index<usize, Output = Self::Input> {
    /// The type of the input.
    type Input = Op;
    /// The number of inputs available.
    fn num_inputs(&self) -> usize;
    /// Get an input.
    fn input(&self, index: usize) -> Option<&Self::Input>;
}

impl<'cook, Op> Index<usize> for OperatorInputs<'cook, Op>
where
    Self: GetInput<'cook, Op>,
{
    type Output = <Self as GetInput<'cook, Op>>::Input;

    fn index(&self, index: usize) -> &Self::Output {
        self.input(index).expect("Invalid input index")
    }
}

#[cfg(not(feature = "python"))]
pub unsafe fn op_info<T: OpInfo>(mut op_info: std::pin::Pin<&mut cxx::OP_CustomOPInfo>) {
    let new_string = std::ffi::CString::new(T::OPERATOR_TYPE).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opType, new_string_ptr);
    let new_string = std::ffi::CString::new(T::OPERATOR_LABEL).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opLabel, new_string_ptr);
    let new_string = std::ffi::CString::new(T::OPERATOR_ICON).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opIcon, new_string_ptr);
    op_info.minInputs = T::MIN_INPUTS as i32;
    op_info.maxInputs = T::MAX_INPUTS as i32;
    let new_string = std::ffi::CString::new(T::AUTHOR_NAME).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.authorName, new_string_ptr);
    let new_string = std::ffi::CString::new(T::AUTHOR_EMAIL).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.authorEmail, new_string_ptr);
    op_info.majorVersion = T::MAJOR_VERSION;
    op_info.minorVersion = T::MINOR_VERSION;
    op_info.cookOnStart = T::COOK_ON_START;
    let callbacks = std::ffi::CString::new(T::PYTHON_CALLBACKS_DAT).unwrap();
    op_info.pythonCallbacksDAT = callbacks.as_ptr();
    std::mem::forget(callbacks); // Callbacks are static
}

#[cfg(feature = "python")]
pub unsafe fn op_info<T: OpInfo + PyMethods + PyGetSets>(
    mut op_info: std::pin::Pin<&mut cxx::OP_CustomOPInfo>,
) {
    let new_string = std::ffi::CString::new(T::OPERATOR_TYPE).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opType, new_string_ptr);
    let new_string = std::ffi::CString::new(T::OPERATOR_LABEL).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opLabel, new_string_ptr);
    let new_string = std::ffi::CString::new(T::OPERATOR_ICON).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.opIcon, new_string_ptr);
    op_info.minInputs = T::MIN_INPUTS as i32;
    op_info.maxInputs = T::MAX_INPUTS as i32;
    let new_string = std::ffi::CString::new(T::AUTHOR_NAME).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.authorName, new_string_ptr);
    let new_string = std::ffi::CString::new(T::AUTHOR_EMAIL).unwrap();
    let new_string_ptr = new_string.as_ptr();
    cxx::setString(op_info.authorEmail, new_string_ptr);
    op_info.majorVersion = T::MAJOR_VERSION;
    op_info.minorVersion = T::MINOR_VERSION;
    op_info.cookOnStart = T::COOK_ON_START;
    let callbacks = std::ffi::CString::new(T::PYTHON_CALLBACKS_DAT).unwrap();
    op_info.pythonCallbacksDAT = callbacks.as_ptr();
    std::mem::forget(callbacks); // Callbacks are static
    py::py_op_info::<T>(op_info);
}

/// Base functionality for all operator types.
pub fn op_init() {
    #[cfg(feature = "tracing")]
    {
        use tracing_subscriber::fmt;
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::EnvFilter;
        use tracing_subscriber::util::{SubscriberInitExt, TryInitError};

        let fmt_layer = if cfg!(target_os = "windows") {
            let mut f = fmt::layer();
            f.set_ansi(false);
            f
        } else {
            fmt::layer()
        };
        let init = tracing_subscriber::registry()
            .with(fmt_layer)
            .with(EnvFilter::from_default_env())
            .try_init();
        match init {
            Ok(_) => {}
            Err(err) => {
                match err {
                    TryInitError { .. } => {}
                    _ => {
                        eprintln!("Failed to initialize tracing: {}", err);
                    }
                }
            }
        }
    }
}
