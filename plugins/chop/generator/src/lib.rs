use td_rs_chop::param::MenuParam;
use td_rs_chop::*;
use td_rs_derive::{Param, Params};

#[derive(Param, Default, Clone, Debug)]
enum Operation {
    #[default]
    Add,
    Multiply,
    Power,
}

#[derive(Params, Default, Clone, Debug)]
struct GeneratorChopParams {
    #[param(label = "Length", page = "Generator")]
    length: u32,
    #[param(label = "Number of Channels", page = "Generator", min = -10.0, max = 10.0)]
    num_channels: u32,
    #[param(label = "Apply Scale", page = "Generator")]
    apply_scale: bool,
    #[param(label = "Scale", page = "Generator")]
    scale: f32,
    #[param(label = "Operation", page = "Generator")]
    operation: Operation,
}

/// Struct representing our CHOP's state
pub struct GeneratorChop {
    params: GeneratorChopParams,
}

/// Impl block providing default constructor for plugin
impl OpNew for GeneratorChop {
    fn new(info: NodeInfo) -> Self {
        Self {
            params: GeneratorChopParams {
                length: 0,
                num_channels: 0,
                apply_scale: false,
                scale: 1.0,
                operation: Operation::Add,
            },
        }
    }
}

impl OpInfo for GeneratorChop {
    const OPERATOR_LABEL: &'static str = "Basic Generator";
    const OPERATOR_TYPE: &'static str = "Basicgenerator";
}

impl Op for GeneratorChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Chop for GeneratorChop {
    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        let params = inputs.params();
        params.enable_param("Scale", self.params.apply_scale);

        for i in 0..output.num_channels() {
            for j in 0..output.num_samples() {
                let cur_value = match self.params.operation {
                    Operation::Add => (i as f32) + (j as f32),
                    Operation::Multiply => (i as f32) * (j as f32),
                    Operation::Power => (i as f32).powf(j as f32),
                };
                let scale = if self.params.apply_scale {
                    self.params.scale
                } else {
                    1.0
                };
                let cur_value = cur_value * scale;
                output[i][j] = cur_value as f32;
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

    fn output_info(&self, _inputs: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        Some(ChopOutputInfo {
            num_channels: self.params.num_channels,
            num_samples: self.params.length,
            start_index: 0,
            ..Default::default()
        })
    }
}

chop_plugin!(GeneratorChop);
