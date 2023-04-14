use std::pin::Pin;
use crate::chop::Chop;
use crate::cxx::ffi::*;
use cxx::ExternType;
use crate::chop;

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
pub mod ffi {
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
    pub struct ChopChannel {
        pub data: Vec<f32>,
    }

    #[derive(Debug, Default)]
    pub struct ParamValue {
        pub name: String,
        pub str_value: String,
        pub double_value: f64,
    }

    #[derive(Debug, Default)]
    pub struct ChopOperatorInputs {
        pub num_inputs: i32,
        pub inputs: Vec<ChopOperatorInput>,
        pub params: Vec<ParamValue>,
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
        include!("BoxDynChop.h");
        type BoxDynChop = Box<dyn crate::chop::Chop>;
        type PtrBoxDynChop = crate::cxx::PtrBoxDynChop;
    }

    unsafe extern "C++" {
        include!("ChopOutput.h");
        pub(crate) type ChopOutput;
        pub fn getNumChannels(&self) -> i32;
        pub fn getNumSamples(&self) -> i32;
        pub fn getSampleRate(&self) -> i32;
        pub fn getStartIndex(&self) -> usize;
        pub fn getChannelNames(&self) -> &[&str];
        pub fn getChannels(self: Pin<&mut ChopOutput>) -> &mut [&mut [f32]];
    }

    unsafe extern "C++" {
        include!("ParameterManager.h");
        pub(crate) type ParameterManager;
        pub fn appendFloat(&self, np: NumericParameter);
        pub fn appendPulse(&self, np: NumericParameter);
    }

    extern "Rust" {
        unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop);
        fn chop_setup_params(chop: &mut BoxDynChop, manager: Pin<&mut ParameterManager>);
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
            output: Pin<&mut ChopOutput>,
            inputs: &ChopOperatorInputs,
        );
        fn chop_get_general_info(chop: &BoxDynChop) -> ChopGeneralInfo;
        fn chop_get_info(chop: &BoxDynChop) -> String;
        fn chop_get_warning(chop: &BoxDynChop) -> String;
        fn chop_get_error(chop: &BoxDynChop) -> String;
        fn chop_get_operator_info() -> OperatorInfo;
        fn chop_new() -> BoxDynChop;
    }
}

// FFI
fn chop_setup_params(chop: &mut Box<dyn Chop>, manager: Pin<&mut ParameterManager>) {
    (**chop).setup_params(&mut chop::ParameterManager::new(manager));
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

fn chop_execute(chop: &mut Box<dyn Chop>, output: Pin<&mut ChopOutput>, inputs: &ChopOperatorInputs) {
    (**chop).execute(&mut chop::ChopOutput::new(output), inputs);
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

extern {
    fn chop_get_operator_info_impl() -> OperatorInfo;
    fn chop_new_impl() -> Box<dyn Chop>;
}

fn chop_get_operator_info() -> OperatorInfo {
    unsafe { chop_get_operator_info_impl() }
}

fn chop_new() -> Box<dyn Chop> {
    unsafe { chop_new_impl() }
}