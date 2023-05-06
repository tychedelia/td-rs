pub mod cxx;
pub mod operator_input;
pub mod parameter_manager;

use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
pub use operator_input::OperatorInput;
pub use parameter_manager::ParameterManager;

use crate::cxx::ffi::{NumericParameter, StringParameter};

/// Trait for defining operator parameters.
pub trait OperatorParams {
    /// Register parameters with the parameter manager.
    fn register(&mut self, parameter_manager: &mut ParameterManager);
    /// Update parameters from operator input.
    fn update(&mut self, input: &OperatorInput);
}

/// Options for creating parameters in derive macro.
/// Not intended for direct use.
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
            min_values: [options.min, 0.0, 0.0, 0.0],
            max_values: [options.max, 0.0, 0.0, 0.0],
            min_sliders: [options.min, 0.0, 0.0, 0.0],
            max_sliders: [options.max, 0.0, 0.0, 0.0],
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
