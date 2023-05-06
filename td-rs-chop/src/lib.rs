use std::ops::{Index, IndexMut};
use std::pin::Pin;
use std::sync::Arc;
use autocxx::prelude::*;
pub use td_rs_base::*;

pub mod cxx;

#[derive(Debug, Default)]
pub struct ChopOutputInfo {
    pub num_channels: u32,
    pub num_samples: u32,
    pub sample_rate: f32,
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

/// A wrapper around a `ChopOutput` that provides a safe interface to the
/// underlying C++ object and writing to the output buffer.
pub struct ChopOutput<'execute> {
    output: Pin<&'execute mut cxx::CHOP_Output>,
}

impl<'execute> ChopOutput<'execute> {
    /// Create a new `ChopOutput` from a pinning reference to a
    /// `ChopOutput`.
    pub fn new(output: Pin<&'execute mut cxx::CHOP_Output>) -> ChopOutput<'execute> {
        Self { output }
    }

    /// Get the number of channels in the output buffer.
    pub fn num_channels(&self) -> usize {
        self.output.numChannels as usize
    }

    /// Get the number of samples in the output buffer.
    pub fn num_samples(&self) -> usize {
        self.output.numSamples as usize
    }

    /// Get the sample rate of the output buffer.
    pub fn sample_rate(&self) -> u32 {
        self.output.sampleRate as u32
    }

    /// Get the start index of the output buffer.
    pub fn start_index(&self) -> usize {
        self.output.startIndex as usize
    }

    pub fn channel(&self, index: usize) -> &[f32] {
        if index >= self.num_channels() {
            panic!("Channel index out of bounds");
        }

        unsafe {
            let channel_ptr = *self.output.channels.offset(index as isize);
            std::slice::from_raw_parts(channel_ptr, self.num_samples() as usize)
        }
    }

    pub fn channel_mut(&mut self, index: usize) -> &mut [f32] {
        if index >= self.num_channels() {
            panic!("Channel index out of bounds");
        }

        unsafe {
            let channel_ptr = *self.output.channels.offset(index as isize);
            std::slice::from_raw_parts_mut(channel_ptr, self.num_samples() as usize)
        }
    }
}

impl Index<usize> for ChopOutput<'_> {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        self.channel(index)
    }
}

impl IndexMut<usize> for ChopOutput<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.channel_mut(index)
    }
}

/// Trait for defining a custom operator.
pub trait Chop: Op {
    fn channel_name(&self, index: usize, input: &OperatorInputs<ChopInput>) -> String {
        String::from("")
    }

    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
    }

    fn execute(&mut self, output: &mut ChopOutput, input: &OperatorInputs<ChopInput>);

    fn general_info(&self, input: &OperatorInputs<ChopInput>) -> ChopGeneralInfo;

    fn output_info(&self, input: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        None
    }
}

#[macro_export]
macro_rules! chop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_chop::cxx::OP_CustomOPInfo;

        #[no_mangle]
        pub extern "C" fn chop_get_plugin_info_impl(mut op_info: Pin<&mut OP_CustomOPInfo>) {
            unsafe {
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_TYPE).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.opType, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_LABEL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.opLabel, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::OPERATOR_ICON).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.opIcon, new_string_ptr);
                op_info.minInputs = <$plugin_ty>::MIN_INPUTS as i32;
                op_info.maxInputs = <$plugin_ty>::MAX_INPUTS as i32;
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_NAME).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.authorName, new_string_ptr);
                let new_string = std::ffi::CString::new(<$plugin_ty>::AUTHOR_EMAIL).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.authorEmail, new_string_ptr);
                op_info.majorVersion = <$plugin_ty>::MAJOR_VERSION;
                op_info.minorVersion = <$plugin_ty>::MINOR_VERSION;
                let new_string = std::ffi::CString::new(<$plugin_ty>::PYTHON_VERSION).unwrap();
                let new_string_ptr = new_string.as_ptr();
                td_rs_chop::cxx::setString(op_info.pythonVersion, new_string_ptr);
                op_info.cookOnStart = <$plugin_ty>::COOK_ON_START;
            }
        }

        #[no_mangle]
        pub extern "C" fn chop_new_impl() -> Box<dyn Chop> {
            Box::new(<$plugin_ty>::new())
        }
    };
}