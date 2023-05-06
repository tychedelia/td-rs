use crate::chop;
use crate::chop::Chop;
use crate::cxx::ffi::*;
use cxx::ExternType;
use std::pin::Pin;
use td_rs_base::cxx::ffi::OperatorInput;
use td_rs_base::cxx::ffi::ParameterManager;

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

    #[namespace = "td_rs_base::ffi"]
    extern "C++" {
        include!("parameter_manager/ParameterManager.h");
        type ParameterManager = td_rs_base::cxx::ffi::ParameterManager;
    }

    #[namespace = "td_rs_base::ffi"]
    extern "C++" {
        include!("operator_input/OperatorInput.h");
        type OperatorInput = td_rs_base::cxx::ffi::OperatorInput;
    }

    extern "C++" {
        include!("BoxDynChop.h");
        type BoxDynChop = Box<dyn crate::chop::Chop>;
        type PtrBoxDynChop = crate::cxx::PtrBoxDynChop;
    }

    unsafe extern "C++" {
        include!("ChopOperatorInput.h");
        pub(crate) type ChopOperatorInput;
        pub fn getInput(&self, idx: usize) -> UniquePtr<ChopInput>;
    }

    unsafe extern "C++" {
        include!("ChopInput.h");
        pub(crate) type ChopInput;
        pub fn getPath(&self) -> &str;
        pub fn getId(&self) -> u32;
        pub fn getNumChannels(&self) -> i32;
        pub fn getNumSamples(&self) -> i32;
        pub fn getSampleRate(&self) -> f64;
        pub fn getStartIndex(&self) -> f64;
        pub fn getChannelNames(&self) -> &[&str];
        pub fn getChannels(&self) -> &[&[f32]];
        pub fn getTotalCooks(&self) -> i64;
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

    extern "Rust" {
        unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop);
        fn chop_setup_params(chop: &mut BoxDynChop, manager: Pin<&mut ParameterManager>);
        fn chop_on_reset(chop: &mut BoxDynChop);
        fn chop_get_num_info_chop_chans(chop: &BoxDynChop) -> i32;
        fn chop_get_info_chop_chan(chop: &BoxDynChop, index: i32) -> ChopInfoChan;
        fn chop_get_output_info(
            chop: &mut BoxDynChop,
            info: &mut ChopOutputInfo,
            chop_input: Pin<&ChopOperatorInput>,
        ) -> bool;
        fn chop_get_channel_name(
            chop: &BoxDynChop,
            index: i32,
            chop_input: Pin<&ChopOperatorInput>,
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
            input: Pin<&OperatorInput>,
            chop_input: Pin<&ChopOperatorInput>,
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
    let params = (**chop).get_params_mut();
    if let Some(mut params) = params {
        let mut mgr = td_rs_base::parameter_manager::ParameterManager::new(manager);
        params.register(&mut mgr);
    }
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
    chop_input: Pin<&ChopOperatorInput>,
) -> bool {
    let chop_input = crate::chop::ChopOperatorInput::new(chop_input);
    (**chop).get_output_info(info, &chop_input)
}

fn chop_get_channel_name(chop: &BoxDynChop, index: i32, chop_input: Pin<&ChopOperatorInput>) -> String {
    let chop_input = crate::chop::ChopOperatorInput::new(chop_input);
    (**chop).get_channel_name(index, &chop_input)
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

fn chop_execute(
    chop: &mut Box<dyn Chop>,
    output: Pin<&mut ChopOutput>,
    input: Pin<&OperatorInput>,
    chop_input: Pin<&ChopOperatorInput>,
) {
    let mut input = td_rs_base::operator_input::OperatorInput::new(input);
    let params = (**chop).get_params_mut();
    if let Some(mut params) = params {
        params.update(&input);
    }
    let chop_input = crate::chop::ChopOperatorInput::new(chop_input);
    (**chop).execute(&mut chop::ChopOutput::new(output), &chop_input);
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

extern "C" {
    fn chop_get_operator_info_impl() -> OperatorInfo;
    fn chop_new_impl() -> Box<dyn Chop>;
}

fn chop_get_operator_info() -> OperatorInfo {
    unsafe { chop_get_operator_info_impl() }
}

fn chop_new() -> Box<dyn Chop> {
    unsafe { chop_new_impl() }
}
