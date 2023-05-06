use crate::cxx::ffi::*;
use std::pin::Pin;

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

impl<'execute> ParameterManager<'execute> {
    pub fn new(
        manager: Pin<&'execute mut crate::cxx::ffi::ParameterManager>,
    ) -> ParameterManager<'execute> {
        Self { manager }
    }

    pub fn append_float(&self, param: NumericParameter) {
        self.manager.appendFloat(param);
    }

    pub fn append_pulse(&self, param: NumericParameter) {
        self.manager.appendPulse(param);
    }

    pub fn append_int(&self, param: NumericParameter) {
        self.manager.appendInt(param);
    }

    pub fn append_xy(&self, param: NumericParameter) {
        self.manager.appendXY(param);
    }

    pub fn append_xyz(&self, param: NumericParameter) {
        self.manager.appendXYZ(param);
    }

    pub fn append_uv(&self, param: NumericParameter) {
        self.manager.appendUV(param);
    }

    pub fn append_uvw(&self, param: NumericParameter) {
        self.manager.appendUVW(param);
    }

    pub fn append_rgb(&self, param: NumericParameter) {
        self.manager.appendRGB(param);
    }

    pub fn append_rgba(&self, param: NumericParameter) {
        self.manager.appendRGBA(param);
    }

    pub fn append_toggle(&self, param: NumericParameter) {
        self.manager.appendToggle(param);
    }

    pub fn append_string(&self, param: StringParameter) {
        self.manager.appendString(param);
    }

    pub fn append_file(&self, param: StringParameter) {
        self.manager.appendFile(param);
    }

    pub fn append_folder(&self, param: StringParameter) {
        self.manager.appendFolder(param);
    }

    pub fn append_dat(&self, param: StringParameter) {
        self.manager.appendDAT(param);
    }

    pub fn append_chop(&self, param: StringParameter) {
        self.manager.appendCHOP(param);
    }

    pub fn append_top(&self, param: StringParameter) {
        self.manager.appendTOP(param);
    }

    pub fn append_object(&self, param: StringParameter) {
        self.manager.appendObject(param);
    }

    pub fn append_menu(&self, param: StringParameter, names: &[&str], labels: &[&str]) {
        self.manager.appendMenu(param, names, labels);
    }

    pub fn append_string_menu(&self, param: StringParameter, names: &[&str], labels: &[&str]) {
        self.manager.appendStringMenu(param, names, labels);
    }

    pub fn append_sop(&self, param: StringParameter) {
        self.manager.appendSOP(param);
    }

    pub fn append_python(&self, param: StringParameter) {
        self.manager.appendPython(param);
    }

    pub fn append_op(&self, param: StringParameter) {
        self.manager.appendOP(param);
    }

    pub fn append_comp(&self, param: StringParameter) {
        self.manager.appendCOMP(param);
    }

    pub fn append_mat(&self, param: StringParameter) {
        self.manager.appendMAT(param);
    }

    pub fn append_panel_comp(&self, param: StringParameter) {
        self.manager.appendPanelCOMP(param);
    }

    pub fn append_header(&self, param: StringParameter) {
        self.manager.appendHeader(param);
    }

    pub fn append_momentary(&self, param: NumericParameter) {
        self.manager.appendMomentary(param);
    }

    pub fn append_wh(&self, param: NumericParameter) {
        self.manager.appendWH(param);
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

pub trait Chop {
    /// Called when pulse is pressed with reset message.
    fn on_reset(&mut self) {}

    /// Called on plugin init to declare parameters required for plugin.
    fn setup_params(&self, manager: &mut ParameterManager) {}

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
    fn get_output_info(&self, info: &mut ChopOutputInfo, input: &OperatorInput) -> bool {
        false
    }

    /// Called for each channel to get the channel's name.
    fn get_channel_name(&self, index: i32, input: &OperatorInput) -> String {
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
    fn execute(&mut self, output: &mut ChopOutput, input: &OperatorInput);

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
