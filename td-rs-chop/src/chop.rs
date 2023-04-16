pub use crate::cxx::ffi::*;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_base::operator_input::OperatorInput;
use td_rs_base::{OperatorParams, ParameterManager};

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

pub struct ChopOperatorInput<'execute> {
    input: Pin<&'execute crate::cxx::ffi::ChopOperatorInput>,
}

impl <'execute> ChopOperatorInput<'execute> {
    pub fn new(input: Pin<&'execute crate::cxx::ffi::ChopOperatorInput>) -> ChopOperatorInput<'execute> {
        Self { input }
    }

    pub fn get_input(&self, idx: usize) -> Option<ChopInput> {
        let input = self.input.getInput(idx);
        Some(ChopInput::new(input))
    }
}

pub struct ChopInput {
    input: cxx::UniquePtr<crate::cxx::ffi::ChopInput>,
}

impl  ChopInput {
    pub fn new(input: cxx::UniquePtr<crate::cxx::ffi::ChopInput>) -> ChopInput {
        Self { input }
    }

    pub fn num_channels(&self) -> u32 {
        self.input.getNumChannels() as u32
    }

    pub fn num_samples(&self) -> u32 {
        self.input.getNumSamples() as u32
    }

    pub fn sample_rate(&self) -> u32 {
        self.input.getSampleRate() as u32
    }

    pub fn start_index(&self) -> usize {
        self.input.getStartIndex() as usize
    }

    pub fn channel_name(&self, idx: usize) -> &[&str] {
        self.input.getChannelNames()
    }

    pub fn channels(&mut self) -> &[&[f32]] {
        self.input.getChannels()
    }
}

pub struct ChopOutput<'execute> {
    output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>,
}

impl<'execute> ChopOutput<'execute> {
    pub fn new(output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>) -> ChopOutput<'execute> {
        Self { output }
    }

    pub fn num_channels(&self) -> u32 {
        self.output.getNumChannels() as u32
    }

    pub fn num_samples(&self) -> u32 {
        self.output.getNumSamples() as u32
    }

    pub fn sample_rate(&self) -> u32 {
        self.output.getSampleRate() as u32
    }

    pub fn start_index(&self) -> usize {
        self.output.getStartIndex()
    }
    pub fn channel_names(&self) -> &[&str] {
        let channel_names = &*self.output.getChannelNames();
        channel_names
    }

    pub fn channels_mut(&mut self) -> &mut [&mut [f32]] {
        let channels = &mut *self.output.as_mut().getChannels();
        channels
    }

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
