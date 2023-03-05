use std::f64::consts::PI;
use std::pin::Pin;
use td_rs_chop::chop::{Chop, ChopInfo, ChopOutput, ParameterManager};
use td_rs_chop::cxx::ffi::*;

/// Struct representing our CHOP's state
pub struct SinChop {
    execute_count: u64,
}

/// Impl block providing default constructor for plugin
impl SinChop {
    pub(crate) fn new() -> Self {
        Self { execute_count: 0 }
    }
}

impl ChopInfo for SinChop {
}

impl Chop for SinChop {

    fn on_reset(&mut self) {
        self.execute_count = 0;
    }

    fn setup_params(&self, manager: &mut ParameterManager) {
        manager.append_pulse(&mut NumericParameter {
            name: "Reset".to_string(),
            label: "Reset".to_string(),
            ..Default::default()
        });

    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        info.num_channels = 3;
        info.num_samples = 100;
        info.start_index = 0;
        info.sample_rate = 60.0;
        true
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
        for chan_index in 0..output.num_channels() {
            let phase = (2.0 * PI) / (chan_index as f64 + 1.0);

            for index in 0..output.num_samples() {
                let percent = (index as f64) / (output.num_samples() as f64);
                let timestep = (self.execute_count as f64) / output.sample_rate() as f64;
                let val = (phase * percent + timestep).sin();

                output.channels_mut()[chan_index as usize][index as usize] = val as f32;
            }
        }

        self.execute_count += 1;
    }

    fn get_general_info(&self) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            timeslice: false,
            input_match_index: 0
        }
    }
}

td_rs_chop::chop_plugin!(SinChop);