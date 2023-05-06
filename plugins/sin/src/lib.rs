use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_chop::*;
use td_rs_derive::Params;

#[derive(Params, Default, Clone)]
struct SinChopParams {
    scale: f64,
}

/// Struct representing our CHOP's state
pub struct SinChop {
    execute_count: u64,
    params: SinChopParams,
}

/// Impl block providing default constructor for plugin
impl SinChop {
    pub(crate) fn new() -> Self {
        Self {
            execute_count: 0,
            params: Default::default(),
        }
    }
}

impl ChopInfo for SinChop {
    const OPERATOR_LABEL: &'static str = "Sin";
}

impl Chop for SinChop {
    fn get_params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, input: &ChopOperatorInput) -> bool {
        info.num_channels = 3;
        info.num_samples = 100;
        info.start_index = 0;
        info.sample_rate = 60.0;
        true
    }

    fn execute(&mut self, output: &mut ChopOutput, input: &ChopOperatorInput) {
        for chan_index in 0..output.num_channels() {
            let phase = (2.0 * PI) / (chan_index as f64 + 1.0);

            for index in 0..output.num_samples() {
                let percent = (index as f64) / (output.num_samples() as f64);
                let timestep = (self.execute_count as f64)
                    / (output.sample_rate() as f64 * self.params.scale);
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
            input_match_index: 0,
        }
    }
}

chop_plugin!(SinChop);
