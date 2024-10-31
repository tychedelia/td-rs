#![feature(min_specialization)]

use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::pin::Pin;

pub use td_rs_base::chop::*;
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
pub struct ChopGeneralInfo {
    pub cook_every_frame: bool,
    pub cook_every_frame_if_asked: bool,
    pub timeslice: bool,
    pub input_match_index: i32,
}

/// A wrapper around a `ChopOutput` that provides a safe interface to the
/// underlying C++ object and writing to the output buffer.
pub struct ChopOutput<'cook> {
    output: Pin<&'cook mut cxx::CHOP_Output>,
}

impl std::fmt::Debug for ChopOutput<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChopOutput")
            .field("num_channels", &self.num_channels())
            .field("num_samples", &self.num_samples())
            .field("sample_rate", &self.sample_rate())
            .field("start_index", &self.start_index())
            .finish()
    }
}

impl<'cook> ChopOutput<'cook> {
    /// Create a new `ChopOutput` from a pinning reference to a
    /// `ChopOutput`.
    pub fn new(output: Pin<&'cook mut cxx::CHOP_Output>) -> ChopOutput<'cook> {
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
            let channel_ptr = *self.output.channels.add(index);
            std::slice::from_raw_parts(channel_ptr, self.num_samples())
        }
    }

    pub fn channel_mut(&mut self, index: usize) -> &mut [f32] {
        if index >= self.num_channels() {
            panic!("Channel index out of bounds");
        }

        unsafe {
            let channel_ptr = *self.output.channels.add(index);
            std::slice::from_raw_parts_mut(channel_ptr, self.num_samples())
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
    fn channel_name(&self, _index: usize, _input: &OperatorInputs<ChopInput>) -> String {
        String::from("")
    }

    fn execute(&mut self, output: &mut ChopOutput, input: &OperatorInputs<ChopInput>);

    fn general_info(&self, input: &OperatorInputs<ChopInput>) -> ChopGeneralInfo;

    fn output_info(&self, _input: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        None
    }
}

#[macro_export]
macro_rules! chop_plugin {
    ($plugin_ty:ty) => {
        use td_rs_chop::cxx::c_void;
        use td_rs_chop::cxx::OP_CustomOPInfo;
        use td_rs_chop::NodeInfo;

        #[no_mangle]
        pub extern "C" fn chop_get_plugin_info_impl(
            mut op_info: std::pin::Pin<&mut OP_CustomOPInfo>,
        ) {
            unsafe {
                td_rs_chop::op_info::<$plugin_ty>(op_info);
            }
        }

        #[no_mangle]
        pub extern "C" fn chop_new_impl(info: NodeInfo) -> Box<dyn Chop> {
            op_init();
            Box::new(<$plugin_ty>::new(info))
        }
    };
}
