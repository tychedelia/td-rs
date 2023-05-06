use std::f64::consts::PI;
use std::pin::Pin;
use std::sync::Arc;
use td_rs_chop::*;
use td_rs_derive::Params;

#[derive(Params, Default, Clone)]
struct GeneratorChopParams {
    #[param(label = "Length", page = "Generator")]
    length: u32,
    #[param(label = "Scale", page = "Generator", min = -10.0, max = 10.0)]
    scale: f32,
    #[param(label = "Apply Offset", page = "Generator")]
    apply_offset: bool,
    #[param(label = "Offset", page = "Generator", min = -10.0, max = 10.0)]
    offset: f32,
}

/// Struct representing our CHOP's state
pub struct GeneratorChop {
    params: GeneratorChopParams,
}

/// Impl block providing default constructor for plugin
impl GeneratorChop {
    pub(crate) fn new() -> Self {
        Self {
            params: GeneratorChopParams {
                apply_scale: true,
                scale: 1.0,
                apply_offset: false,
                offset: 0.0,
            }
        }
    }
}

impl ChopInfo for GeneratorChop {
    const OPERATOR_LABEL: &'static str = "Basic Generator";
    const OPERATOR_TYPE: &'static str = "Basicgenerator";
}

impl Chop for GeneratorChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs) {
        inputs.enable_param("Scale", true);
        inputs.enable_param("Offset", true);

        if input.num_inputs() == 1 {
            let input = &inputs[0];
            for i in 0..output.num_channels() {
                for j in 0..output.num_samples() {
                    output[i][j] = input[i][j] * self.params.scale + self.params.offset;
                }
            }
        }
    }

    fn general_info(&self, inputs: &OperatorInputs) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: false,
            cook_every_frame_if_asked: false,
            timeslice: false,
            input_match_index: 0,
        }
    }


    fn channel_name(&self, index: usize, inputs: &OperatorInputs) -> String {
        format!("chan{}", index)
    }

    fn output_info(&self, inputs: &OperatorInputs) -> Option<ChopOutputInfo> {

    }
}

chop_plugin!(GeneratorChop);
