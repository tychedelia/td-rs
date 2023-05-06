use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::pin::Pin;
use td_rs_chop::chop::{Chop, ChopInfo};
use td_rs_chop::cxx::ffi::*;

#[derive(Default)]
pub struct Lissajous {
    point_count: f64,
    freq_x: f64,
    freq_y: f64,
    mod_freq_x: f64,
    mod_freq_y: f64,
    phi: f64,
}

impl Lissajous
    where Self: Chop
{
    pub fn new() -> Self {
        Default::default()
    }

    fn map_params(&mut self, inputs: &ChopOperatorInputs) {
        let params: HashMap<String, &ParamValue> = inputs.params.iter()
            .map(|x| (x.name.clone(), x.clone()))
            .collect();

        self.point_count = params[&"Points".to_string()].double_value;
        self.freq_x = params[&"Xfreq".to_string()].double_value;
        self.freq_y = params[&"Yfreq".to_string()].double_value;
        self.mod_freq_x = params[&"Xmod".to_string()].double_value;
        self.mod_freq_y = params[&"Ymod".to_string()].double_value;
        self.phi = params[&"Phi".to_string()].double_value;
    }
}

struct LissajousParams {
    point_count: f64,
    freq_x: f64,
    freq_y: f64,
    mod_freq_x: f64,
    mod_freq_y: f64,
    phi: f64,
}

impl ChopInfo for Lissajous {}

impl Chop for Lissajous {
    fn setup_params(&self) -> ChopParams {
        ChopParams {
            numeric_params: vec![
                NumericParameter {
                    name: "Points".to_string(),
                    label: "Point Count".to_string(),
                    default_values: [1000.0, 0.0, 0.0, 0.0],
                    max_values: [1000.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
                NumericParameter {
                    name: "Xfreq".to_string(),
                    label: "X Frequency".to_string(),
                    default_values: [4.0, 0.0, 0.0, 0.0],
                    max_values: [255.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
                NumericParameter {
                    name: "Yfreq".to_string(),
                    label: "Y Frequency".to_string(),
                    default_values: [7.0, 0.0, 0.0, 0.0],
                    max_values: [255.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
                NumericParameter {
                    name: "Xmod".to_string(),
                    label: "X Mod".to_string(),
                    default_values: [3.0, 0.0, 0.0, 0.0],
                    max_values: [255.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
                NumericParameter {
                    name: "Ymod".to_string(),
                    label: "Y Mod".to_string(),
                    default_values: [2.0, 0.0, 0.0, 0.0],
                    max_values: [255.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
                NumericParameter {
                    name: "Phi".to_string(),
                    label: "Phi".to_string(),
                    default_values: [15.0, 0.0, 0.0, 0.0],
                    max_values: [255.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                },
            ],
            pulse_params: vec![
                PuleParameter{
                    name: "Reset".to_string(),
                    label: "Reset".to_string(),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    }

    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool {
        info.num_channels = 2; // x + y
        info.num_samples =  1000; //self.point_count as u32;
        info.start_index = 0;
        info.sample_rate = 60.0;
        true
    }

    fn execute(&mut self, output: Pin<&mut ChopOutput>, inputs: &ChopOperatorInputs) {
        self.map_params(inputs);

        for p_i in 0..self.point_count as usize {
            let angle = map_range((0.0, self.point_count as f64), (0.0, PI * 2.0), p_i as f64);
            let x = f64::sin(self.phi.to_radians()  + (angle * self.freq_x)) * f64::cos(angle * self.mod_freq_x);
            let y = f64::sin(angle * self.freq_y) * f64::cos(angle * self.mod_freq_y);

            output.channels[0].data.push(x as f32);
            output.channels[1].data.push(y as f32);
        }
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


fn map_range<T>(from_range: (T, T), to_range: (T, T), s: T) ->  T
    where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

td_rs_chop::chop_plugin!(Lissajous);