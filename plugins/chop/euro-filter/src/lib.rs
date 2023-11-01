use crate::filter::OneEuroImpl;

use td_rs_chop::*;
use td_rs_derive::Params;

mod filter;

#[derive(Params, Default, Clone, Debug)]
struct EuroFilterChopParams {
    #[param(label = "Cutoff Frequency (Hz)", page = "EuroFilter")]
    min_cutoff: f64,
    #[param(label = "Speed Coefficient", page = "EuroFilter")]
    beta: f64,
    #[param(label = "Slope Cutoff Frequency (Hz)", page = "EuroFilter")]
    d_cutoff: f64,
}

/// Struct representing our CHOP's state
#[derive(Default)]
pub struct EuroFilterChop {
    filters: Vec<OneEuroImpl>,
    params: EuroFilterChopParams,
}

impl OpNew for EuroFilterChop {
    fn new(_info: NodeInfo) -> Self {
        Default::default()
    }
}

impl OpInfo for EuroFilterChop {
    const OPERATOR_TYPE: &'static str = "Eurofilter";
    const OPERATOR_LABEL: &'static str = "Euro Filter";
    const MIN_INPUTS: usize = 1;
    const MAX_INPUTS: usize = 1;
}

impl Op for EuroFilterChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Chop for EuroFilterChop {
    fn channel_name(&self, index: usize, _inputs: &OperatorInputs<ChopInput>) -> String {
        format!("chan{}", index)
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        if let Some(input) = &inputs.input(0) {
            for filter in &mut self.filters {
                filter.change_input(
                    input.num_samples() as f64,
                    self.params.min_cutoff,
                    self.params.beta,
                    self.params.d_cutoff,
                );
            }

            for _ in self.filters.len()..input.num_channels() {
                self.filters.push(OneEuroImpl::new(
                    input.num_samples() as f64,
                    self.params.min_cutoff,
                    self.params.beta,
                    self.params.d_cutoff,
                ));
            }

            let mut input_sample_idx = 0;
            for i in 0..output.num_channels() {
                for j in 0..output.num_samples() {
                    input_sample_idx = (input_sample_idx + 1) % input.num_samples();
                    output[i][j] = self.filters[i].filter(input[i][input_sample_idx] as f64) as f32;
                }
            }
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: true,
            timeslice: true,
            input_match_index: 0,
        }
    }

    fn output_info(&self, _inputs: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        None
    }
}

chop_plugin!(EuroFilterChop);
