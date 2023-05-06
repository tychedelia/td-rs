use crate::{ChopGeneralInfo, ChopInfoChan, ChopInfoDatEntries, ChopInfoDatSize, ChopOperatorInputs, ChopOutput, ChopOutputInfo, ChopParams, StringParameter};

pub mod sin;
pub mod arc;
pub mod grid;
pub mod lissa;

/// Interface to implement a TouchDesigner CHOP Operator.
///
/// Methods are called from TD C++ plugin via FFI mapping.
///
/// Implement this trait, providing an execute method, and override any other methods as needed
/// to support your operator.
///
/// See documentation on the [C++ base class](../cpp/CHOP_CPlusPlusBase.h) for more information
/// about how TD calls these methods.
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
