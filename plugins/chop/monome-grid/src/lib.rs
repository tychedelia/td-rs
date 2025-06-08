use monome::{KeyDirection, Monome, MonomeDevice, MonomeEvent};
use td_rs_chop::cxx::OP_Inputs;
use td_rs_chop::*;
use td_rs_derive::{Param, Params};

#[derive(Param, Default, Clone, Debug)]
enum Operation {
    #[default]
    Add,
    Multiply,
    Power,
}

#[derive(Params, Default, Clone, Debug, Eq, PartialEq)]
struct MonomeGridParams {
    #[param(label = "Prefix", page = "Grid", default = "/touchdesigner")]
    prefix: String,
    #[param(label = "Hold", page = "Grid")]
    hold: bool,
}

/// Struct representing our CHOP's state
#[derive(Debug)]
pub struct MonomeGrid {
    params: MonomeGridParams,
    prev_params: MonomeGridParams,
    device: Option<Monome>,
    grid: [bool; 128],
}

/// Impl block providing default constructor for plugin
impl OpNew for MonomeGrid {
    fn new(_info: NodeInfo) -> Self {
        Self {
            params: MonomeGridParams {
                prefix: "/touchdesigner".to_string(),
                hold: false,
            },
            prev_params: Default::default(),
            device: None,
            grid: [false; 128],
        }
    }
}

impl OpInfo for MonomeGrid {
    const OPERATOR_LABEL: &'static str = "Monome Grid";
    const OPERATOR_TYPE: &'static str = "Monomegrid";
}

impl Op for MonomeGrid {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }
}

impl Chop for MonomeGrid {
    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        if self.params != self.prev_params || self.device.is_none() {
            self.prev_params = self.params.clone();
            let device = match Monome::new(&self.params.prefix) {
                Ok(device) => device,
                Err(err) => {
                    self.set_error(&format!("Error connecting to monome: {}", err));
                    return;
                }
            };
            self.device = Some(device);
        }

        if let Some(ref mut device) = &mut self.device {
            while let Some(event) = device.poll() {
                match event {
                    MonomeEvent::GridKey { x, y, direction } => {
                        let index = (y * 16 + x) as usize;
                        if self.params.hold {
                            if matches!(direction, KeyDirection::Down) {
                                self.grid[index] = !self.grid[index];
                            }
                        } else {
                            self.grid[index] = !self.grid[index];
                        }
                    }
                    _ => {}
                }
            }

            device.set_all(&self.grid);
        }

        for i in 0..output.num_channels() {
            output[i][0] = if self.grid[i] { 1.0 } else { 0.0 };
        }
    }

    fn general_info(&self, _inputs: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: true,
            cook_every_frame_if_asked: true,
            timeslice: false,
            input_match_index: 0,
        }
    }

    fn channel_name(&self, index: usize, _inputs: &OperatorInputs<ChopInput>) -> String {
        format!("grid{}", index)
    }

    fn output_info(&self, _inputs: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        Some(ChopOutputInfo {
            num_channels: 128,
            num_samples: 1,
            start_index: 0,
            ..Default::default()
        })
    }
}

chop_plugin!(MonomeGrid);
