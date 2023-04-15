pub mod parameter_manager;
pub mod operator_input;
pub mod cxx;

pub use operator_input::OperatorInput;
pub use parameter_manager::ParameterManager;

use crate::cxx::ffi::{NumericParameter, StringParameter};

pub trait OperatorParams {
    fn register(&mut self, parameter_manager: &mut ParameterManager);
    fn update(&mut self, input: &OperatorInput);
}

pub struct ParamOptions {
    pub name: String,
    pub label: String,
    pub page: String,
    pub min: f64,
    pub max: f64,
}

impl From<ParamOptions> for NumericParameter {
    fn from(options : ParamOptions) -> Self {
        NumericParameter {
            name: options.name,
            label: options.label,
            page: options.page,
            min_values: [options.min, 0.0, 0.0, 0.0],
            max_values: [options.max, 0.0, 0.0, 0.0],
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

macro_rules! impl_param_int {
    ( $t:ty ) => {
        impl Param for $t {
            fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
                parameter_manager.append_float(options.into());
            }

            fn update(&mut self, name: &str, input: &OperatorInput) {
                *self = input.get_int(name, 0) as $t;
            }
        }
    }
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
                parameter_manager.append_float(options.into());
            }

            fn update(&mut self, name: &str, input: &OperatorInput) {
                *self = input.get_float(name, 0) as $t;
            }
        }
    }
}

impl_param_float!(f32);
impl_param_float!(f64);

impl Param for String {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager) {
        parameter_manager.append_string(options.into());
    }

    fn update(&mut self, name: &str, input: &OperatorInput) {
        *self = input.get_string(name).to_string();
    }
}

pub trait Param {
    fn register(&self, options: ParamOptions, parameter_manager: &mut ParameterManager);
    fn update(&mut self, name: &str, input: &OperatorInput);
}