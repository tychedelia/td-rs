mod chop;
mod sin_chop;
mod arc_chop;

use crate::chop::Chop;
use crate::ffi::*;
use crate::sin_chop::SinChop;
use cxx::ExternType;
use crate::arc_chop::ArcChop;

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
    pub struct PuleParameter {
        pub name: String,
        pub label: String,
        pub page: String,
    }

    #[derive(Debug, Default)]
    pub struct StringParameter {
        pub name: String,
        pub label: String,
        pub page: String,
        pub default_value: String,
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
        pub numeric_params: Vec<NumericParameter>,
        pub string_params: Vec<StringParameter>,
        pub pulse_params: Vec<PuleParameter>,
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
        pub start_index: usize,
    }

    #[derive(Debug, Default)]
    pub struct ChopOutput {
        pub channels: Vec<ChopChannel>,
        pub num_channels: i32,
        pub num_samples: i32,
        pub sample_rate: i32,
    }

    #[derive(Debug, Default)]
    pub struct ChopInfoChan {
        name: String,
        value: f32,
    }

    #[derive(Debug, Default)]
    pub struct ChopInfoDatSize {
        rows: i32,
        columns: i32,
    }

    #[derive(Debug, Default)]
    pub struct ChopInfoDatEntries {
        values: Vec<String>,
    }

    #[derive(Debug, Default)]
    pub struct ChopGeneralInfo {
        pub cook_every_frame: bool,
        pub cook_every_frame_if_asked: bool,
        pub timeslice: bool,
        pub input_match_index: i32,
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
        fn chop_get_num_info_chop_chans(chop: &BoxDynChop) -> i32;
        fn chop_get_info_chop_chan(chop: &BoxDynChop, index: i32) -> ChopInfoChan;
        fn chop_get_output_info(
            chop: &mut BoxDynChop,
            info: &mut ChopOutputInfo,
            inputs: &ChopOperatorInputs,
        ) -> bool;
        fn chop_get_channel_name(
            chop: &BoxDynChop,
            index: i32,
            inputs: &ChopOperatorInputs,
        ) -> String;
        fn chop_get_info_dat_size(chop: &BoxDynChop, size: &mut ChopInfoDatSize) -> bool;
        fn chop_get_info_dat_entries(
            chop: &BoxDynChop,
            index: i32,
            num_entries: i32,
            entries: &mut ChopInfoDatEntries,
        );
        fn chop_execute(
            chop: &mut BoxDynChop,
            output: &mut ChopOutput,
            inputs: &ChopOperatorInputs,
        );
        fn chop_get_general_info(chop: &BoxDynChop) -> ChopGeneralInfo;
        fn chop_get_info(chop: &BoxDynChop) -> String;
        fn chop_get_warning(chop: &BoxDynChop) -> String;
        fn chop_get_error(chop: &BoxDynChop) -> String;
        fn chop_new() -> BoxDynChop;
    }
}

// FFI

fn chop_get_params(chop: &mut BoxDynChop) -> ChopParams {
    (**chop).get_params()
}

fn chop_on_reset(chop: &mut Box<dyn Chop>) {
    (**chop).on_reset();
}

fn chop_get_num_info_chop_chans(chop: &BoxDynChop) -> i32 {
    (**chop).get_num_info_chop_chans()
}

fn chop_get_info_chop_chan(chop: &BoxDynChop, index: i32) -> ChopInfoChan {
    (**chop).get_info_chop_chan(index)
}

fn chop_get_output_info(
    chop: &mut Box<dyn Chop>,
    info: &mut ChopOutputInfo,
    inputs: &ChopOperatorInputs,
) -> bool {
    (**chop).get_output_info(info, inputs)
}

fn chop_get_channel_name(chop: &BoxDynChop, index: i32, inputs: &ChopOperatorInputs) -> String {
    (**chop).get_channel_name(index, inputs)
}

fn chop_get_info_dat_size(chop: &BoxDynChop, size: &mut ChopInfoDatSize) -> bool {
    (**chop).get_info_dat_size(size)
}

fn chop_get_info_dat_entries(
    chop: &BoxDynChop,
    index: i32,
    num_entries: i32,
    entries: &mut ChopInfoDatEntries,
) {
    (**chop).get_info_dat_entries(index, num_entries, entries)
}

fn chop_execute(chop: &mut Box<dyn Chop>, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
    (**chop).execute(output, inputs)
}

fn chop_get_general_info(chop: &BoxDynChop) -> ChopGeneralInfo {
    (**chop).get_general_info()
}

fn chop_get_info(chop: &BoxDynChop) -> String {
    (**chop).get_info()
}

fn chop_get_warning(chop: &BoxDynChop) -> String {
    (**chop).get_warning()
}

fn chop_get_error(chop: &BoxDynChop) -> String {
    (**chop).get_error()
}

unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop) {
    std::ptr::drop_in_place(ptr.0);
}

// Custom Chop Impl

fn chop_get_operator_info() -> OperatorInfo {
    OperatorInfo {
        operator_type: "test1".to_string(),
        operator_label: "test2".to_string(),
        operator_icon: "test3".to_string(),
        min_inputs: 0,
        max_inputs: 0,
        author_name: "test4".to_string(),
        author_email: "test5".to_string(),
        major_version: 0,
        minor_version: 0,
        python_version: "".to_string(),
        cook_on_start: false,
    }
}

fn chop_new() -> Box<dyn Chop> {
    Box::new(ArcChop::new())
}