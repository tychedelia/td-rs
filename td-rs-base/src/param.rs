use crate::chop::ChopInput;
use crate::sop::{Color, SopInput};
use crate::{cxx, ParamInputs};
use ref_cast::RefCast;
use std::ffi;
use std::ffi::{c_char, CString};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::pin::Pin;

/// A numeric parameter.
// TODO: switch to enum to describe parameter types
#[derive(Debug)]
pub struct NumericParameter {
    /// The name of the parameter.
    pub name: String,
    /// The label of the parameter.
    pub label: String,
    /// The page of the parameter.
    pub page: String,

    /// The default values of the parameter.
    pub default_values: [f64; 4],
    /// The minimum values of the parameter.
    pub min_values: [f64; 4],
    /// The maximum values of the parameter.
    pub max_values: [f64; 4],
    /// Whether to clamp the minimum values of the parameter.
    pub clamp_mins: [bool; 4],
    /// Whether to clamp the maximum values of the parameter.
    pub clamp_maxes: [bool; 4],
    /// The minimum slider values of the parameter.
    pub min_sliders: [f64; 4],
    /// The maximum slider values of the parameter.
    pub max_sliders: [f64; 4],
}

impl Default for NumericParameter {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            label: "".to_string(),
            page: "".to_string(),
            default_values: [0.0; 4],
            min_values: [0.0; 4],
            max_values: [1.0; 4],
            clamp_mins: [false; 4],
            clamp_maxes: [false; 4],
            min_sliders: [0.0; 4],
            max_sliders: [1.0; 4],
        }
    }
}

/// Trait for defining operator parameters.
pub trait OperatorParams {
    /// Register parameters with the parameter manager.
    fn register(&mut self, parameter_manager: &mut ParameterManager);
    /// Update parameters from operator input.
    fn update(&mut self, inputs: &ParamInputs);
}

impl From<NumericParameter> for cxx::OP_NumericParameter {
    fn from(param: NumericParameter) -> Self {
        cxx::OP_NumericParameter {
            name: ffi::CString::new(param.name).unwrap().into_raw(),
            label: ffi::CString::new(param.label).unwrap().into_raw(),
            page: ffi::CString::new(param.page).unwrap().into_raw(),
            defaultValues: param.default_values,
            minValues: param.min_values,
            maxValues: param.max_values,
            clampMins: param.clamp_mins,
            clampMaxes: param.clamp_maxes,
            minSliders: param.min_sliders,
            maxSliders: param.max_sliders,
            reserved: Default::default(),
        }
    }
}

/// A string parameter.
#[derive(Debug, Default)]
pub struct StringParameter {
    /// The name of the parameter.
    pub name: String,
    /// The label of the parameter.
    pub label: String,
    /// The page of the parameter.
    pub page: String,
    /// The default value of the parameter.
    pub default_value: String,
}

impl From<StringParameter> for cxx::OP_StringParameter {
    fn from(param: StringParameter) -> Self {
        cxx::OP_StringParameter {
            name: ffi::CString::new(param.name).unwrap().into_raw(),
            label: ffi::CString::new(param.label).unwrap().into_raw(),
            page: ffi::CString::new(param.page).unwrap().into_raw(),
            defaultValue: ffi::CString::new(param.default_value).unwrap().into_raw(),
            reserved: Default::default(),
        }
    }
}

/// Manager for registering parameters with TouchDesigner.
pub struct ParameterManager<'cook> {
    manager: Pin<&'cook mut crate::cxx::OP_ParameterManager>,
}

impl<'cook> ParameterManager<'cook> {
    /// Create a new parameter manager. Should not be called by
    /// users.
    pub fn new(manager: Pin<&'cook mut crate::cxx::OP_ParameterManager>) -> ParameterManager<'cook> {
        Self { manager }
    }

    /// Append a float parameter.
    pub fn append_float(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendFloat(&param, 1);
    }

    /// Append a pulse parameter.
    pub fn append_pulse(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendPulse(&param);
    }

    /// Append an integer parameter.
    pub fn append_int(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendInt(&param, 1);
    }

    /// Append an xy parameter.
    pub fn append_xy(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXY(&param);
    }

    /// Append an xyz parameter.
    pub fn append_xyz(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXYZ(&param);
    }

    /// Append a uv parameter.
    pub fn append_uv(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUV(&param);
    }

    /// Append a uvw parameter.
    pub fn append_uvw(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUVW(&param);
    }

    /// Append a rgb parameter.
    pub fn append_rgb(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGB(&param);
    }

    /// Append a rgba parameter.
    pub fn append_rgba(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGBA(&param);
    }

    /// Append a toggle parameter.
    pub fn append_toggle(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendToggle(&param);
    }

    /// Append a string parameter.
    pub fn append_string(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendString(&param);
    }

    /// Append a file parameter.
    pub fn append_file(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFile(&param);
    }

    /// Append a folder parameter.
    pub fn append_folder(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFolder(&param);
    }

    /// Append a dat reference parameter.
    pub fn append_dat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendDAT(&param);
    }

    /// Append a chop reference parameter.
    pub fn append_chop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCHOP(&param);
    }

    /// Append a top reference parameter.
    pub fn append_top(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendTOP(&param);
    }

    /// Append an object reference parameter.
    pub fn append_object(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendObject(&param);
    }

    pub fn append_menu(&mut self, param: StringParameter, names: &[String], labels: &[String]) {
        assert_eq!(names.len(), labels.len());
        let n_items = names.len() as i32;
        let c_strings: Vec<CString> = names
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();

        let name_ptrs: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();
        let names: *mut *const c_char = name_ptrs.as_ptr() as *mut *const c_char;

        let c_strings: Vec<CString> = labels
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();

        let label_ptrs: Vec<*const c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();
        let labels: *mut *const c_char = label_ptrs.as_ptr() as *mut *const c_char;

        let param = param.into();
        unsafe {
            self.manager
                .as_mut()
                .appendMenu(&param, n_items, names, labels);
        }
    }

    // pub fn append_string_menu(&mut self, param: StringParameter, names: &[&str], labels: &[&str]) {
    //     self.manager.as_mut().appendStringMenu(&param, names.len(), names.as_ptr(), labels.as_ptr());
    // }

    /// Append a sop reference parameter.
    pub fn append_sop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendSOP(&param);
    }

    /// Append a python parameter.
    pub fn append_python(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPython(&param);
    }

    /// Append an op reference parameter.
    pub fn append_op(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendOP(&param);
    }

    /// Append a comp reference parameter.
    pub fn append_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCOMP(&param);
    }

    /// Append a mat reference parameter.
    pub fn append_mat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendMAT(&param);
    }

    /// Append a panel comp parameter.
    pub fn append_panel_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPanelCOMP(&param);
    }

    /// Append a header parameter.
    pub fn append_header(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendHeader(&param);
    }

    /// Append a momentary parameter.
    pub fn append_momentary(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendMomentary(&param);
    }

    /// Append a wh parameter.
    pub fn append_wh(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendWH(&param);
    }
}

/// Options for creating parameters in derive macro.
/// Not intended for direct use.
#[derive(Debug)]
pub struct ParamOptions {
    pub name: String,
    pub label: String,
    pub page: String,
    pub min: f64,
    pub max: f64,
    pub min_slider: f64,
    pub max_slider: f64,
    pub clamp: bool,
    pub default: f64,
}

impl From<ParamOptions> for NumericParameter {
    fn from(options: ParamOptions) -> Self {
        NumericParameter {
            name: options.name,
            label: options.label,
            page: options.page,
            default_values: [options.default; 4],
            min_values: [options.min; 4],
            max_values: [options.max; 4],
            clamp_mins: [options.clamp; 4],
            clamp_maxes: [options.clamp; 4],
            min_sliders: [options.min_slider; 4],
            max_sliders: [options.max_slider; 4],
        }
    }
}

impl From<ParamOptions> for StringParameter {
    fn from(options: ParamOptions) -> Self {
        StringParameter {
            name: options.name,
            label: options.label,
            page: options.page,
            ..Default::default()
        }
    }
}

/// Trait for implementing parameter types.
pub trait Param {
    /// Register parameter with the parameter manager.
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager);
    /// Update parameter from operator input.
    fn update(&mut self, name: &str, inputs: &ParamInputs);
}

macro_rules! impl_param_int {
    ( $t:ty ) => {
        impl Param for $t {
            fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
                let mut param: NumericParameter = options.into();
                param.default_values = [*self as f64, 0.0, 0.0, 0.0];
                parameter_manager.append_int(param);
            }

            fn update(&mut self, name: &str, inputs: &ParamInputs) {
                *self = inputs.get_int(name, 0) as $t;
            }
        }
    };
}

impl_param_int!(i8);
impl_param_int!(i16);
impl_param_int!(i32);
impl_param_int!(i64);
impl_param_int!(i128);
impl_param_int!(isize);
impl_param_int!(u8);
impl_param_int!(u16);
impl_param_int!(u32);
impl_param_int!(u64);
impl_param_int!(u128);
impl_param_int!(usize);

macro_rules! impl_param_float {
    ( $t:ty ) => {
        impl Param for $t {
            fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
                let mut param: NumericParameter = options.into();
                param.default_values = [*self as f64, 0.0, 0.0, 0.0];
                parameter_manager.append_float(param);
            }

            fn update(&mut self, name: &str, inputs: &ParamInputs) {
                *self = inputs.get_float(name, 0) as $t;
            }
        }
    };
}

impl_param_float!(f32);
impl_param_float!(f64);

impl Param for String {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: StringParameter = options.into();
        param.default_value = self.clone();
        parameter_manager.append_string(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_string(name).to_string();
    }
}

impl Param for rgb::RGB8 {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values = [self.r as f64, self.g as f64, self.b as f64, 0.0];
        parameter_manager.append_rgb(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = rgb::RGB8::new(
            inputs.get_int(name, 0) as u8,
            inputs.get_int(name, 1) as u8,
            inputs.get_int(name, 2) as u8,
        );
    }
}

impl Param for rgb::RGB16 {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values = [self.r as f64, self.g as f64, self.b as f64, 0.0];
        parameter_manager.append_rgb(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = rgb::RGB16::new(
            inputs.get_int(name, 0) as u16,
            inputs.get_int(name, 1) as u16,
            inputs.get_int(name, 2) as u16,
        );
    }
}

impl Param for rgb::RGBA8 {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values = [self.r as f64, self.g as f64, self.b as f64, self.a as f64];
        parameter_manager.append_rgba(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = rgb::RGBA8::new(
            inputs.get_int(name, 0) as u8,
            inputs.get_int(name, 1) as u8,
            inputs.get_int(name, 2) as u8,
            inputs.get_int(name, 3) as u8,
        );
    }
}

impl Param for rgb::RGBA16 {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values = [self.r as f64, self.g as f64, self.b as f64, self.a as f64];
        parameter_manager.append_rgba(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = rgb::RGBA16::new(
            inputs.get_int(name, 0) as u16,
            inputs.get_int(name, 1) as u16,
            inputs.get_int(name, 2) as u16,
            inputs.get_int(name, 3) as u16,
        );
    }
}

/// A parameter wrapping a `PathBuf` that will be registered as a folder parameter.
pub struct FolderParam(PathBuf);

impl Deref for FolderParam {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FolderParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Param for FolderParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: StringParameter = options.into();
        param.default_value = self.to_string_lossy().to_string();
        parameter_manager.append_folder(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        self.0 = PathBuf::from(inputs.get_string(name));
    }
}

/// A parameter wrapping a `PathBuf` that will be registered as a file parameter.
#[derive(Default, Clone, Debug)]
pub struct FileParam(PathBuf);

impl Deref for FileParam {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Param for FileParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: StringParameter = options.into();
        param.default_value = self.to_string_lossy().to_string();
        parameter_manager.append_file(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        self.0 = PathBuf::from(inputs.get_string(name));
    }
}

impl Param for PathBuf {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: StringParameter = options.into();
        param.default_value = self.to_string_lossy().to_string();
        parameter_manager.append_file(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = PathBuf::from(inputs.get_string(name));
    }
}

impl Param for bool {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values[0] = true as usize as f64;
        parameter_manager.append_toggle(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_toggle(name);
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Pulse;

impl Param for Pulse {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: NumericParameter = options.into();
        parameter_manager.append_pulse(param);
    }

    fn update(&mut self, _name: &str, _inputs: &ParamInputs) {}
}

/// A chop parameter.
#[derive(Default, Debug, Clone)]
pub struct ChopParam {
    pub(crate) input: Option<*const cxx::OP_CHOPInput>,
}

impl Param for ChopParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: StringParameter = options.into();
        parameter_manager.append_chop(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_chop(name);
    }
}

impl ChopParam {
    /// Get the chop input for this parameter, if it exists.
    pub fn input(&self) -> Option<&ChopInput> {
        self.input
            .map(|input| unsafe { ChopInput::ref_cast(&*input) })
    }
}

#[derive(Default, Debug, Clone)]
pub struct SopParam {
    pub(crate) input: Option<*const cxx::OP_SOPInput>,
}

impl Param for SopParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: StringParameter = options.into();
        parameter_manager.append_sop(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_sop(name);
    }
}

impl SopParam {
    /// Get the sop input for this parameter, if it exists.
    pub fn input(&self) -> Option<&SopInput> {
        self.input
            .map(|input| unsafe { SopInput::ref_cast(&*input) })
    }
}

#[derive(Default, Debug, Clone)]
pub struct TopParam {
    pub(crate) input: Option<*const cxx::OP_TOPInput>,
}

impl Param for TopParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: StringParameter = options.into();
        parameter_manager.append_top(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_top(name);
    }
}

impl TopParam {
    /// Get the top input for this parameter, if it exists.
    pub fn input(&self) -> Option<&crate::top::TopInput> {
        self.input
            .map(|input| unsafe { crate::top::TopInput::ref_cast(&*input) })
    }
}

#[derive(Default, Debug, Clone)]
pub struct DatParam {
    pub(crate) input: Option<*const cxx::OP_DATInput>,
}

impl Param for DatParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: StringParameter = options.into();
        parameter_manager.append_dat(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_dat(name);
    }
}

impl DatParam {
    /// Get the dat input for this parameter, if it exists.
    pub fn input(&self) -> Option<&crate::dat::DatInput> {
        self.input
            .map(|input| unsafe { crate::dat::DatInput::ref_cast(&*input) })
    }
}

#[cfg(feature = "python")]
impl Param for *mut pyo3_ffi::PyObject {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param = options.into();
        parameter_manager.append_python(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        // Ensure that the old object is decref'd
        unsafe {
            pyo3_ffi::Py_XDECREF(*self);
        }
        *self = inputs.get_python(name);
    }
}

pub trait MenuParam {
    fn names() -> Vec<String>;
    fn labels() -> Vec<String>;
}

impl Param for Color {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param = options.into();
        parameter_manager.append_rgba(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        let [r, g, b, a] = inputs.get_double_arr::<4>(name);
        *self = (r, g, b, a).into();
    }
}
