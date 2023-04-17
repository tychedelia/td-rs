use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};
use std::path::PathBuf;
use std::pin::Pin;
use td_rs_chop::*;
use td_rs_derive::Params;

#[derive(Params, Default)]
pub struct Lissajous {
    #[param(label = "Point Count", min = 1.0, max = 1000.0)]
    point_count: f64,
    #[param(label = "Frequency X", min = 0.0, max = 10.0)]
    freq_x: f64,
    #[param(label = "Frequency Y", min = 0.0, max = 10.0)]
    freq_y: f64,
    #[param(label = "Mod Frequency X", min = 0.0, max = 10.0)]
    mod_freq_x: f64,
    #[param(label = "Mod Frequency Y", min = 0.0, max = 10.0)]
    mod_freq_y: f64,
    #[param(label = "Phi", min = 0.0, max = 360.0)]
    phi: f64,
    file: PathBuf,
}

impl Lissajous
where
    Self: Chop,
{
    pub fn new() -> Self {
        Self {
            point_count: 100.0,
            freq_x: 4.0,
            freq_y: 5.0,
            mod_freq_x: 3.0,
            mod_freq_y: 2.0,
            phi: 15.0,
            file: Default::default(),
        }
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
    fn get_params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(self))
    }


    fn get_output_info(&self, info: &mut ChopOutputInfo, inputs: &ChopOperatorInput) -> bool {
        info.num_channels = 2; // x + y
        info.num_samples = 1000; //self.point_count as u32;
        info.start_index = 0;
        info.sample_rate = 60.0;
        true
    }

    fn execute(&mut self, output: &mut ChopOutput, input: &ChopOperatorInput) {
        for p_i in 0..self.point_count as usize {
            let angle = map_range((0.0, self.point_count as f64), (0.0, PI * 2.0), p_i as f64);
            let x = f64::sin(self.phi.to_radians() + (angle * self.freq_x))
                * f64::cos(angle * self.mod_freq_x);
            let y = f64::sin(angle * self.freq_y) * f64::cos(angle * self.mod_freq_y);

            output.channels_mut()[0][p_i] = x as f32;
            output.channels_mut()[1][p_i] = y as f32;
        }
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

fn map_range<T>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

chop_plugin!(Lissajous);
