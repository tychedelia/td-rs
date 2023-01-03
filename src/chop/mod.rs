use cxx::ExternType;
use crate::chop::ffi::*;

/// Interface to implement a TouchDesigner CHOP Operator.
///
/// Methods are called from TD C++ plugin via FFI mapping.
///
/// Implement this trait, providing an execute method, and override any other methods as needed
/// to support your operator.
///
/// See documentation on the [C++ base class](../cpp/CHOP_CPlusPlusBase.h) for more information
/// about how TD calls these methods.
pub trait ChopInfo {
    const OPERATOR_TYPE: &'static str = "";
    const OPERATOR_LABEL: &'static str = "";
    const OPERATOR_ICON: &'static str = "";
    const MIN_INPUTS: i32 = 0;
    const MAX_INPUTS: i32 = 0;
    const AUTHOR_NAME: &'static str = "";
    const AUTHOR_EMAIL: &'static str = "";
    const MAJOR_VERSION: i32 = 0;
    const MINOR_VERSION: i32 = 0;
    const PYTHON_VERSION: &'static str = "";
    const COOK_ON_START: bool = false;
}

pub trait Chop {

    /// Called when pulse is pressed with reset message.
    fn on_reset(&mut self) {

    }

    /// Called on plugin init to declare parameters required for plugin.
    fn get_params(&self) -> ChopParams {
        ChopParams::default()
    }

    /// Called on plugin init to register the number of info chop channels.
    fn get_num_info_chop_chans(&self) -> i32 {
        0
    }

    /// Called for each info chop channel on init.
    fn get_info_chop_chan(&self, index: i32) -> ChopInfoChan {
        // must be implemented if getNumInfoCHOPChans is called with > 0
        unimplemented!()
    }

    /// Called on plugin init to register output channels, if any.
    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        false
    }

    /// Called for each channel to get the channel's name.
    fn get_channel_name(&self, index: i32, inputs: &ChopOperatorInputs) -> String {
        format!("chan{}", index)
    }

    /// Called on plugin init to declare the size of the info dat.
    fn get_info_dat_size(&self, size: &mut ChopInfoDatSize) -> bool {
        false
    }

    /// Called for each row in the info dat to populate the dat with entries.
    fn get_info_dat_entries(&self, index: i32, num_entries: i32, entries: &mut ChopInfoDatEntries) {
        // must be implemented if GetInfoDatSize returns true
        unimplemented!()
    }

    /// Execute the chop, filling the output channels.
    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs);

    /// Called on plugin init for the chop's general info.
    fn get_general_info(&self) -> ChopGeneralInfo {
        ChopGeneralInfo::default()
    }

    /// Called each cook to provide an info string for the chop (or blank).
    fn get_info(&self) -> String {
        "".to_string()
    }

    /// Called each cook to provide a warning (or blank).
    fn get_warning(&self) -> String {
        "".to_string()
    }

    /// Called each cook to provide an error (or blank).
    fn get_error(&self) -> String {
        "".to_string()
    }
}

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
        type PtrBoxDynChop = crate::chop::PtrBoxDynChop;
    }

    extern "Rust" {
        unsafe fn dyn_chop_drop_in_place(ptr: PtrBoxDynChop);
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

#[macro_export]
macro_rules! chop_plugin {
    ($plugin_ty:ty) => {
        use td_rs::chop::ffi::OperatorInfo;

        pub extern "C" fn chop_get_operator_info() -> OperatorInfo {
            OperatorInfo {
                operator_type: <$plugin_ty>::OPERATOR_TYPE.to_string(),
                operator_label: <$plugin_ty>::OPERATOR_LABEL.to_string(),
                operator_icon: <$plugin_ty>::OPERATOR_ICON.to_string(),
                min_inputs: <$plugin_ty>::MIN_INPUTS,
                max_inputs: <$plugin_ty>::MAX_INPUTS,
                author_name: <$plugin_ty>::AUTHOR_NAME.to_string(),
                author_email: <$plugin_ty>::AUTHOR_EMAIL.to_string(),
                major_version: <$plugin_ty>::MAJOR_VERSION,
                minor_version: <$plugin_ty>::MINOR_VERSION,
                python_version: <$plugin_ty>::PYTHON_VERSION.to_string(),
                cook_on_start: <$plugin_ty>::COOK_ON_START,
            }
        }


        pub extern "C" fn chop_new() -> Box<dyn Chop> {
            Box::new(<$plugin_ty>::new())
        }
    }
}