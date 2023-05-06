use std::ffi;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::pin::Pin;
use ref_cast::RefCast;
use crate::chop::ChopInput;
use crate::{cxx, ParamInputs};

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
pub struct ParameterManager<'execute> {
    manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>,
}

impl<'execute> ParameterManager<'execute> {
    /// Create a new parameter manager. Should not be called by
    /// users.
    pub fn new(
        mut manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>
    ) -> ParameterManager {
        Self {
            manager
        }
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

    // pub fn append_menu(&mut self, param: StringParameter, names: &[&str], labels: &[&str]) {
    //     self.manager.as_mut().appendMenu(&param, names, labels);
    // }

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
}

impl From<ParamOptions> for NumericParameter {
    fn from(options: ParamOptions) -> Self {
        NumericParameter {
            name: options.name,
            label: options.label,
            page: options.page,
            min_values: [options.min; 4],
            max_values: [options.max; 4],
            ..Default::default()
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
                parameter_manager.append_int(param);            }

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
        param.default_values = [
            self.r as f64,
            self.g as f64,
            self.b as f64,
            0.0,
        ];
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
        param.default_values = [
            self.r as f64,
            self.g as f64,
            self.b as f64,
            0.0,
        ];
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
        param.default_values = [
            self.r as f64,
            self.g as f64,
            self.b as f64,
            self.a as f64,
        ];
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
        param.default_values = [
            self.r as f64,
            self.g as f64,
            self.b as f64,
            self.a as f64,
        ];
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
pub struct Folder(PathBuf);

impl Deref for Folder {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Folder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Param for Folder {
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
pub struct File(PathBuf);

impl Deref for File {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for File {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Param for File {
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

impl Param for ChopParam {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let param: StringParameter = options.into();
        parameter_manager.append_chop(param);
    }

    fn update(&mut self, name: &str, inputs: &ParamInputs) {
        *self = inputs.get_chop(name);
    }
}


/// A chop parameter.
#[derive(Default, Clone)]
pub struct ChopParam {
    pub(crate) input: Option<*const cxx::OP_CHOPInput>,
}

impl ChopParam {
    /// Get the chop input for this parameter, if it exists.
    pub fn input(&self) -> Option<&ChopInput> {
        if let Some(input) = self.input {
            Some(unsafe { ChopInput::ref_cast(&*input) })
        } else {
            None
        }
    }
}