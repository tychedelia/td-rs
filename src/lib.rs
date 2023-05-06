mod chop;
mod sin_chop;

use cxx::ExternType;
use crate::ffi::*;
use crate::chop::Chop;
use crate::sin_chop::SinChop;

unsafe impl ExternType for Box<dyn Chop> {
    type Id = cxx::type_id!("BoxDynChop");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct PtrBoxDynChop(*mut Box<dyn Chop>);
unsafe impl ExternType for PtrBoxDynChop {
    type Id = cxx::type_id!("PtrBoxDynChop");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {
    #[derive(Debug, Default)]
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

    #[derive(Debug, Default)]
    pub struct OperatorInfo {
        pub operator_type: String,
        pub operator_label: String,
        pub operator_icon: String,
        pub min_inputs: i32,
        pub max_inputs: i32,
        pub author_name: String,
        pub author_email: String,
        pub major_version: i32,
        pub minor_version: i32,
        pub python_version: String,
        pub cook_on_start: bool,
    }

    #[derive(Debug, Default)]
    pub struct ChopParams {
        pub params: Vec<NumericParameter>,
    }

    #[derive(Debug, Default)]
    pub struct ChopChannel {
        pub data: Vec<f32>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOperatorInputs {
        pub num_inputs: i32,
        pub inputs: Vec<ChopOperatorInput>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOperatorInput {
        pub path: String,
        pub id: u32,
        pub num_channels: u32,
        pub num_samples: u32,
        pub sample_rate: f64,
        pub start_index: f64,
        pub channels: Vec<ChopChannel>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOutputInfo {
        pub num_channels: u32,
        pub num_samples: u32,
        pub sample_rate: f64,
    }

    #[derive(Debug, Default)]
    pub struct ChopOutput {
        pub channels: Vec<ChopChannel>,
        pub num_channels: i32,
        pub num_samples: i32,
        pub sample_rate: i32,
    }

    extern "C++" {
        include!("td-rs/cpp/BoxDynChop.h");
        type BoxDynChop = Box<dyn crate::chop::Chop>;
        type PtrBoxDynChop = crate::PtrBoxDynChop;
    }

    extern "Rust" {
        unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop);
        fn chop_get_operator_info() -> OperatorInfo;
        fn chop_get_params(chop: &mut BoxDynChop) -> ChopParams;
        fn chop_on_reset(chop: &mut BoxDynChop);
        fn chop_get_output_info(chop: &mut BoxDynChop, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool;
        fn chop_execute(chop: &mut BoxDynChop, output: &mut ChopOutput, inputs: &ChopOperatorInputs);
        fn chop_new() -> BoxDynChop;
    }
}

fn chop_get_operator_info() -> OperatorInfo {
   todo!()
}

fn chop_get_params(chop: &mut BoxDynChop) -> ChopParams {
    (**chop).get_params()
}

fn chop_on_reset(chop: &mut Box<dyn Chop>) {
    (**chop).on_reset();
}

fn chop_get_output_info(chop: &mut Box<dyn Chop>, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
    (**chop).get_output_info(info, inputs)
}

fn chop_execute(chop: &mut Box<dyn Chop>, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
    (**chop).execute(output, inputs)
}

unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop) {
    std::ptr::drop_in_place(ptr.0);
}

fn chop_new() -> Box<dyn Chop> {
    Box::new(SinChop::new())
}