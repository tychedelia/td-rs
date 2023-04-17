pub use crate::cxx::ffi::*;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_base::operator_input::OperatorInput;
use td_rs_base::{OperatorParams, ParameterManager};

/// Trait for defining metadata for a chop operator.
pub trait ChopInfo {
    /// The type of the operator.
    const OPERATOR_TYPE: &'static str = "";
    /// The label of the operator.
    const OPERATOR_LABEL: &'static str = "";
    /// The icon of the operator.
    const OPERATOR_ICON: &'static str = "";
    /// The minimum number of inputs the operator accepts.
    const MIN_INPUTS: i32 = 0;
    /// The maximum number of inputs the operator accepts.
    const MAX_INPUTS: i32 = 0;
    /// The author name of the operator.
    const AUTHOR_NAME: &'static str = "";
    /// The author email of the operator.
    const AUTHOR_EMAIL: &'static str = "";
    /// The major version of the operator.
    const MAJOR_VERSION: i32 = 0;
    /// The minor version of the operator.
    const MINOR_VERSION: i32 = 0;
    /// The python version of the operator.
    const PYTHON_VERSION: &'static str = "";
    /// Whether to cook on start.
    const COOK_ON_START: bool = false;
}

/// A wrapper around a `ChopOperatorInput` that provides a safe interface to the
/// underlying C++ object and retrieving individual inputs.
pub struct ChopOperatorInput<'execute> {
    input: Pin<&'execute crate::cxx::ffi::ChopOperatorInput>,
}

impl <'execute> ChopOperatorInput<'execute> {
    /// Create a new `ChopOperatorInput` from a pinning reference to a
    /// `ChopOperatorInput`.
    pub fn new(input: Pin<&'execute crate::cxx::ffi::ChopOperatorInput>) -> ChopOperatorInput<'execute> {
        Self { input }
    }

    /// Get the input at the given index.
    pub fn get_input(&self, idx: usize) -> Option<ChopInput> {
        let input = self.input.getInput(idx);
        Some(ChopInput::new(input))
    }
}

/// A wrapper around a `ChopInput` that provides a safe interface to the
/// underlying C++ object and reading from the input buffer.
pub struct ChopInput {
    input: cxx::UniquePtr<crate::cxx::ffi::ChopInput>,
}

impl  ChopInput {
    /// Create a new `ChopInput` from a pinning reference to a
    /// `ChopInput`.
    pub fn new(input: cxx::UniquePtr<crate::cxx::ffi::ChopInput>) -> ChopInput {
        Self { input }
    }

    /// Get the number of channels in the input buffer.
    pub fn num_channels(&self) -> u32 {
        self.input.getNumChannels() as u32
    }

    /// Get the number of samples in the input buffer.
    pub fn num_samples(&self) -> u32 {
        self.input.getNumSamples() as u32
    }

    /// Get the sample rate of the input buffer.
    pub fn sample_rate(&self) -> u32 {
        self.input.getSampleRate() as u32
    }

    /// Get the start index of the input buffer.
    pub fn start_index(&self) -> usize {
        self.input.getStartIndex() as usize
    }

    /// Get the channel names of the input buffer by index.
    pub fn channel_name(&self, idx: usize) -> &[&str] {
        self.input.getChannelNames()
    }

    /// Get the channel data of the input buffer.
    pub fn channels(&mut self) -> &[&[f32]] {
        self.input.getChannels()
    }
}

/// A wrapper around a `ChopOutput` that provides a safe interface to the
/// underlying C++ object and writing to the output buffer.
pub struct ChopOutput<'execute> {
    output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>,
}

impl<'execute> ChopOutput<'execute> {
    /// Create a new `ChopOutput` from a pinning reference to a
    /// `ChopOutput`.
    pub fn new(output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>) -> ChopOutput<'execute> {
        Self { output }
    }

    /// Get the number of channels in the output buffer.
    pub fn num_channels(&self) -> u32 {
        self.output.getNumChannels() as u32
    }

    /// Get the number of samples in the output buffer.
    pub fn num_samples(&self) -> u32 {
        self.output.getNumSamples() as u32
    }

    /// Get the sample rate of the output buffer.
    pub fn sample_rate(&self) -> u32 {
        self.output.getSampleRate() as u32
    }

    /// Get the start index of the output buffer.
    pub fn start_index(&self) -> usize {
        self.output.getStartIndex()
    }

    /// Get the channel names of the output buffer.
    pub fn channel_names(&self) -> &[&str] {
        let channel_names = &*self.output.getChannelNames();
        channel_names
    }

    /// Get the output buffer.
    pub fn channels_mut(&mut self) -> &mut [&mut [f32]] {
        let channels = &mut *self.output.as_mut().getChannels();
        channels
    }

    /// Set a value in the output buffer.
    pub fn set_channel_output(&mut self, channel: usize, idx: usize, val: f32) {
        self.output.as_mut().getChannels()[channel][idx] = val;
    }
}

/// Trait for defining a custom operator.
pub trait Chop {
    /// Called when pulse is pressed with reset message.
    fn on_reset(&mut self) {}

    /// Called on plugin init to declare parameters required for plugin.
    fn get_params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        None
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
    fn get_output_info(&self, info: &mut ChopOutputInfo, input: &ChopOperatorInput) -> bool {
        false
    }

    /// Called for each channel to get the channel's name.
    fn get_channel_name(&self, index: i32, input: &ChopOperatorInput) -> String {
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
    fn execute(&mut self, output: &mut ChopOutput, input: &ChopOperatorInput);

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
