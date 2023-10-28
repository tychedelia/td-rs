


use td_rs_chop::*;
use td_rs_derive::Params;

#[derive(Params, Default, Clone)]
struct FilterChopParams {
    #[param(label = "Apply Scale", page = "Filter")]
    apply_scale: bool,
    #[param(label = "Scale", page = "Filter", min = - 10.0, max = 10.0)]
    scale: f32,
    #[param(label = "Apply Offset", page = "Filter")]
    apply_offset: bool,
    #[param(label = "Offset", page = "Filter", min = - 10.0, max = 10.0)]
    offset: f32,
}

/// Struct representing our CHOP's state
pub struct FilterChop {
    params: FilterChopParams,
}

/// Impl block providing default constructor for plugin
impl FilterChop {
    pub(crate) fn new() -> Self {
        Self {
            params: FilterChopParams {
                apply_scale: true,
                scale: 1.0,
                apply_offset: false,
                offset: 0.0,
            },
        }
    }
}

impl OpInfo for FilterChop {
    const OPERATOR_TYPE: &'static str = "Basicfilter";
    const OPERATOR_LABEL: &'static str = "Basic Filter";
    const MIN_INPUTS: usize = 1;
    const MAX_INPUTS: usize = 1;
}

impl Op for FilterChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Chop for FilterChop {
    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        let params = inputs.params();
        params.enable_param("Scale", true);
        params.enable_param("Offset", true);

        if let Some(input) = &inputs.input(0) {
            for i in 0..output.num_channels() {
                for j in 0..output.num_samples() {
                    output[i][j] = input[i][j] * self.params.scale + self.params.offset;
                }
            }
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            timeslice: false,
            input_match_index: 0,
        }
    }

    fn channel_name(&self, index: usize, _inputs: &OperatorInputs<ChopInput>) -> String {
        format!("chan{}", index)
    }
}

chop_plugin!(FilterChop);
