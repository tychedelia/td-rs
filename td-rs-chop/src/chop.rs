use std::pin::Pin;
use crate::cxx::ffi::*;

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

pub struct ParameterManager<'execute> {
    manager: Pin<&'execute mut crate::cxx::ffi::ParameterManager>,
}

impl <'execute> ParameterManager<'execute> {
    pub fn new(manager: Pin<&'execute mut crate::cxx::ffi::ParameterManager>) -> ParameterManager<'execute> {
        Self {
            manager
        }
    }

    pub fn append_float(&self, param: NumericParameter) {
        self.manager.appendFloat(param);
    }

    pub fn append_pulse(&self, param: NumericParameter) {
        self.manager.appendPulse(param);
    }
}

pub struct ChopOutput<'execute> {
    output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>,
}

impl <'execute> ChopOutput<'execute> {
    pub fn new(output: Pin<&'execute mut crate::cxx::ffi::ChopOutput>) -> ChopOutput<'execute> {
        Self {
            output
        }
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

    pub fn channels_mut(&mut self) -> &mut[&mut [f32]] {
        let channels = &mut *self.output.as_mut().getChannels();
        channels
    }

    pub fn set_channel_output(&mut self, channel: usize, idx: usize, val: f32) {
        self.output.as_mut().getChannels()[channel][idx] = val;
    }
}

pub trait Chop {
    /// Called when pulse is pressed with reset message.
    fn on_reset(&mut self) {

    }

    /// Called on plugin init to declare parameters required for plugin.
    fn setup_params(&self, manager: &mut ParameterManager) {

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
