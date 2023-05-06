pub mod cxx;

use std::ffi;
use std::ops::{Deref, DerefMut, Index};
use std::path::PathBuf;
use std::pin::Pin;
use autocxx::cxx::UniquePtr;
use crate::cxx::OP_CHOPInput;
use ref_cast::RefCast;

pub trait Plugin {
    fn num_info_chop_chans(&self) -> usize {
        0
    }

    fn info_popup_string(&self) -> String {
        String::from("")
    }

    fn error_string(&self) -> String {
        String::from("")
    }

    fn warning_string(&self) -> String {
        String::from("")
    }

    fn info_dat_entry(&self, index: usize, entry_index: usize) -> String {
        String::from("")
    }

    fn info_dat_size(&self) -> (u32, u32) {
        (0, 0)
    }

    fn info_chop_chan(&self, index: usize) -> (String, f32) {
        unimplemented!()
    }

    fn pulse_pressed(&mut self, name: &str) {}
}

#[derive(Debug)]
pub struct NumericParameter {
    pub name: String,
    pub label: String,
    pub page: String,

    pub default_values: [f64; 4],
    pub min_values: [f64; 4],
    pub max_values: [f64; 4],
    pub clamp_mins: [bool; 4],
    pub clamp_maxes: [bool; 4],
    pub min_sliders: [f64; 4],
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

#[derive(Debug, Default)]
pub struct StringParameter {
    pub name: String,
    pub label: String,
    pub page: String,
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

pub struct ParameterManager<'execute> {
    manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>,
}

impl<'execute> ParameterManager<'execute> {
    pub fn new(
        mut manager: Pin<&'execute mut crate::cxx::OP_ParameterManager>
    ) -> ParameterManager {
        Self {
            manager
        }
    }

    pub fn append_float(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendFloat(&param, 1);
    }

    pub fn append_pulse(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendPulse(&param);
    }

    pub fn append_int(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendInt(&param, 1);
    }

    pub fn append_xy(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXY(&param);
    }

    pub fn append_xyz(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendXYZ(&param);
    }

    pub fn append_uv(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUV(&param);
    }

    pub fn append_uvw(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendUVW(&param);
    }

    pub fn append_rgb(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGB(&param);
    }

    pub fn append_rgba(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendRGBA(&param);
    }

    pub fn append_toggle(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendToggle(&param);
    }

    pub fn append_string(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendString(&param);
    }

    pub fn append_file(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFile(&param);
    }

    pub fn append_folder(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendFolder(&param);
    }

    pub fn append_dat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendDAT(&param);
    }

    pub fn append_chop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCHOP(&param);
    }

    pub fn append_top(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendTOP(&param);
    }

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

    pub fn append_sop(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendSOP(&param);
    }

    pub fn append_python(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPython(&param);
    }

    pub fn append_op(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendOP(&param);
    }

    pub fn append_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendCOMP(&param);
    }

    pub fn append_mat(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendMAT(&param);
    }

    pub fn append_panel_comp(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendPanelCOMP(&param);
    }

    pub fn append_header(&mut self, param: StringParameter) {
        let param = param.into();
        self.manager.as_mut().appendHeader(&param);
    }

    pub fn append_momentary(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendMomentary(&param);
    }

    pub fn append_wh(&mut self, param: NumericParameter) {
        let param = param.into();
        self.manager.as_mut().appendWH(&param);
    }
}

/// Input to an operator, which can be used to get parameters, channels,
/// and other information.
pub struct OperatorInput<'execute> {
    input: &'execute crate::cxx::OP_Inputs,
}

impl<'execute> OperatorInput<'execute> {
    /// Create a new operator input.
    pub fn new(input: &'execute crate::cxx::OP_Inputs) -> OperatorInput<'execute> {
        Self { input }
    }

    /// Get a float parameter.
    pub fn get_float(&self, name: &str, index: usize) -> f64 {
        unsafe { self.input.getParDouble(ffi::CString::new(name).unwrap().into_raw(), index as i32) }
    }

    /// Get an integer parameter.
    pub fn get_int(&self, name: &str, index: usize) -> i32 {
        unsafe { self.input.getParInt(ffi::CString::new(name).unwrap().into_raw(), index as i32) }
    }

    /// Get a string parameter.
    pub fn get_string(&self, name: &str) -> &str {
        unsafe {
            let res = self.input.getParString(ffi::CString::new(name).unwrap().into_raw());
            ffi::CStr::from_ptr(res).to_str().unwrap()
        }
    }

    pub fn get_toggle(&self, name: &str) -> bool {
        unsafe { self.input.getParInt(ffi::CString::new(name).unwrap().into_raw(), 0) != 0 }
    }

    pub fn enable_param(&self, name: &str, enable: bool) {
        unsafe {
            self.input.enablePar(ffi::CString::new(name).unwrap().into_raw(), enable);
        }
    }

    pub fn num_inputs(&self) -> usize {
        self.input.getNumInputs() as usize
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct ChopInput {
    input: OP_CHOPInput,
}

impl ChopInput {
    pub fn num_channels(&self) -> usize {
        self.input.numChannels as usize
    }

    pub fn channel(&self, index: usize) -> &[f32] {
        if index >= self.num_channels() {
            panic!("index out of bounds");
        }

        unsafe { std::slice::from_raw_parts(*self.input.channelData.offset(index as isize), self.input.numSamples as usize) }
    }
}

impl<'execute> Index<usize> for OperatorInput<'execute> {
    type Output = ChopInput;

    fn index(&self, index: usize) -> &'execute Self::Output {
        if index >= self.num_inputs() {
            panic!("index out of bounds");
        }

        let input = unsafe { &*self.input.getInputCHOP(index as i32) };
        ChopInput::ref_cast(input)
    }
}

impl Index<usize> for ChopInput {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        self.channel(index)
    }
}

/// Trait for defining operator parameters.
pub trait OperatorParams {
    /// Register parameters with the parameter manager.
    fn register(&mut self, parameter_manager: &mut ParameterManager);
    /// Update parameters from operator input.
    fn update(&mut self, input: &OperatorInput);
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
    fn update(&mut self, name: &str, input: &OperatorInput);
}

macro_rules! impl_param_int {
    ( $t:ty ) => {
        impl Param for $t {
            fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
                let mut param: NumericParameter = options.into();
                param.default_values = [*self as f64, 0.0, 0.0, 0.0];
                parameter_manager.append_int(param);            }

            fn update(&mut self, name: &str, input: &OperatorInput) {
                *self = input.get_int(name, 0) as $t;
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

            fn update(&mut self, name: &str, input: &OperatorInput) {
                *self = input.get_float(name, 0) as $t;
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = input.get_string(name).to_string();
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = rgb::RGB8::new(
            input.get_int(name, 0) as u8,
            input.get_int(name, 1) as u8,
            input.get_int(name, 2) as u8,
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = rgb::RGB16::new(
            input.get_int(name, 0) as u16,
            input.get_int(name, 1) as u16,
            input.get_int(name, 2) as u16,
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = rgb::RGBA8::new(
            input.get_int(name, 0) as u8,
            input.get_int(name, 1) as u8,
            input.get_int(name, 2) as u8,
            input.get_int(name, 3) as u8,
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = rgb::RGBA16::new(
            input.get_int(name, 0) as u16,
            input.get_int(name, 1) as u16,
            input.get_int(name, 2) as u16,
            input.get_int(name, 3) as u16,
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        self.0 = PathBuf::from(input.get_string(name));
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

    fn update(&mut self, name: &str, input: &OperatorInput) {
        self.0 = PathBuf::from(input.get_string(name));
    }
}

impl Param for PathBuf {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: StringParameter = options.into();
        param.default_value = self.to_string_lossy().to_string();
        parameter_manager.append_file(param);
    }

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = PathBuf::from(input.get_string(name));
    }
}

impl Param for bool {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        let mut param: NumericParameter = options.into();
        param.default_values[0] = true as usize as f64;
        parameter_manager.append_toggle(param);
    }

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = input.get_toggle(name);
    }
}