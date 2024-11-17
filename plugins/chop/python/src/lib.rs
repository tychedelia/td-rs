#![feature(min_specialization)]

use pyo3::impl_::pyclass::{PyClassImpl, PyMethods};
use pyo3::prelude::PyAnyMethods;
use pyo3::{pyclass, pymethods, Bound, PyAny, PyResult};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use td_rs_chop::cxx::AsPlugin;
use td_rs_chop::*;
use td_rs_derive::*;
use td_rs_derive_py::PyOp;

#[derive(Param, Default, Debug)]
enum PythonChopShape {
    #[default]
    Sine,
    Square,
    Ramp,
}

#[derive(Params, Default, Debug)]
struct PythonChopParams {
    #[param(label="Speed", min=-10.0, max=10.0, default=1.0)]
    speed: f32,
    #[param(label="Scale", min=-10.0, max=10.0, default=1.0)]
    scale: f32,
    #[param(label = "Shape")]
    shape: PythonChopShape,
    #[param(label = "Reset")]
    reset: Pulse,
}

#[derive(PyOp)]
#[pyclass]
pub struct PythonChop {
    info: Arc<Mutex<NodeInfo>>,
    #[pyo3(get, set)]
    speed: f32,
    #[pyo3(get)]
    execute_count: u32,
    offset: f32,
    params: PythonChopParams,
}

#[pymethods]
impl PythonChop {
    pub fn hello_world(&self, a: i32, b: i32, absTime: Bound<'_, PyAny>) -> PyResult<i32> {
        let field_value = absTime.getattr("frame")?;
        // print the pointer to the execute_count field
        println!("execute_count: {:?}", &self.execute_count as *const u32);
        let frames: i32 = field_value.extract()?;
        println!(
            "Hello, world! execute={} frames={}",
            self.execute_count, frames
        );
        Ok(a + b)
    }
}

impl OpNew for PythonChop {
    fn new(info: NodeInfo) -> Self {
        Self {
            info: Arc::new(Mutex::new(info)),
            speed: 1.0,
            execute_count: 0,
            offset: 0.0,
            params: Default::default(),
        }
    }
}

impl OpInfo for PythonChop {
    const OPERATOR_TYPE: &'static str = "Customsignalpython";
    const OPERATOR_LABEL: &'static str = "Custom Signal Python";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
    const PYTHON_CALLBACKS_DAT: &'static str = "
# This is an example callbacks DAT.
#
# op - The OP that is doing the callback
# curSpeed - The current speed value the node will be using.
#
# Change the 0.0 to make the speed get adjusted by this callback.
def getSpeedAdjust(op, curSpeed):
    return curSpeed + 0.0
";
}

impl Op for PythonChop {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn info_dat(&self) -> Option<Box<&dyn InfoDat>> {
        Some(Box::new(self))
    }

    fn info_chop(&self) -> Option<Box<&dyn InfoChop>> {
        Some(Box::new(self))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reset" {}
    }
}

impl Chop for PythonChop {
    fn channel_name(&self, _index: usize, _input: &OperatorInputs<ChopInput>) -> String {
        "chan1".to_string()
    }

    fn execute(&mut self, output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        self.execute_count += 1;
        if inputs.num_inputs() > 0 {
            inputs.params().enable_param("Speed", false);
            inputs.params().enable_param("Reset", false);
            inputs.params().enable_param("Shape", false);
            if let Some(input) = inputs.input(0) {
                let num_samples = output.num_samples();
                let num_channels = output.num_channels();
                for channel in 0..num_channels {
                    let input_channel = input.channel(channel);
                    let output_channel = output.channel_mut(channel);
                    for sample in 0..num_samples {
                        output_channel[sample] = input_channel[sample] * self.params.scale;
                    }
                }
            }
        } else {
            inputs.params().enable_param("Speed", true);
            inputs.params().enable_param("Reset", true);
            inputs.params().enable_param("Shape", true);
            // Apply Python class modifications
            self.params.speed *= self.speed;

            let info = self.info.lock().unwrap();
            let arg_tuple = info 
                .context()
                .create_arguments_tuple(1);

            unsafe {
                pyo3_ffi::PyTuple_SET_ITEM(
                    arg_tuple,
                    1,
                    pyo3_ffi::PyFloat_FromDouble(self.params.speed as std::ffi::c_double),
                );
                let res = info.context().call_python_callback(
                    "getSpeedAdjust",
                    arg_tuple,
                    std::ptr::null_mut(),
                );
                if !res.is_null() {
                    if pyo3_ffi::PyFloat_Check(res) != 0 {
                        self.params.speed = pyo3_ffi::PyFloat_AsDouble(res) as f32;
                    }
                    pyo3_ffi::Py_DECREF(res);
                }
            }

            let phase = 2.0 * std::f32::consts::PI / output.num_channels() as f32;
            let num_samples = output.num_samples();
            let num_channels = output.num_channels();
            let step = self.params.speed * 0.01;
            for channel in 0..num_channels {
                let mut offset = self.offset + phase * channel as f32;
                let v = match self.params.shape {
                    PythonChopShape::Sine => offset.sin(),
                    PythonChopShape::Square => {
                        if (offset % 1.0).abs() > 0.5 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    PythonChopShape::Ramp => (offset % 1.0).abs(),
                };
                let v = v * self.params.scale;

                let output_channel = output.channel_mut(channel);
                for sample in 0..num_samples {
                    output_channel[sample] = v;
                    offset += step;
                }
            }
            self.offset += step * num_samples as f32;
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

    fn output_info(&self, input: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        if input.num_inputs() > 0 {
            None
        } else {
            Some(ChopOutputInfo {
                num_channels: 1,
                sample_rate: 120.0,
                num_samples: 1,
                start_index: 0,
            })
        }
    }
}

impl InfoChop for PythonChop {
    fn size(&self) -> usize {
        2
    }

    fn channel(&self, index: usize) -> (String, f32) {
        match index {
            0 => ("execute_count".to_string(), self.execute_count as f32),
            1 => ("offset".to_string(), self.offset),
            _ => panic!("Invalid channel index"),
        }
    }
}

impl InfoDat for PythonChop {
    fn size(&self) -> (u32, u32) {
        (2, 2)
    }

    fn entry(&self, index: usize, entry_index: usize) -> String {
        match (index, entry_index) {
            (0, 0) => "executeCount".to_string(),
            (0, 1) => "offset".to_string(),
            (1, 0) => self.execute_count.to_string(),
            (1, 1) => self.offset.to_string(),
            (_, _) => panic!("Invalid entry index"),
        }
    }
}

chop_plugin!(PythonChop);
