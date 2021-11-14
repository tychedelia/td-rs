use std::process::Output;
use crate::ffi::*;

#[cxx::bridge]
mod ffi {
    #[derive(Debug, Default)]
    pub struct NumericParameter {
        pub name: String,
        pub label: String,
        pub page: String,

        pub default_values: [f64; 4],
        pub min_values: [f64; 4],
        pub max_values: [f64; 4],
        pub clamp_mins: [bool; 4],
        pub clamp_maxes: [bool; 4],

        pub min_sliders: [f64; 4],
        pub max_sliders: [f64; 4],
    }

    #[derive(Debug, Default)]
    pub struct OperatorInfo {
        pub operator_type: String,
        pub operator_label: String,
        pub operator_icon: String,
        pub min_inputs: i32,
        pub max_inputs: i32,
        pub author_name: String,
        pub author_email: String,
        pub major_version: i32,
        pub minor_version: i32,
        pub python_version: String,
        pub cook_on_start: bool,
    }

    #[derive(Debug, Default)]
    struct Chop {
        pub info: OperatorInfo,
        pub params: Vec<NumericParameter>,
    }

    #[derive(Debug, Default)]
    pub struct ChopChannel {
        pub data: Vec<f32>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOperatorInputs {
        pub num_inputs: i32,
        pub inputs: Vec<ChopOperatorInput>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOperatorInput {
        pub path: String,
        pub id: u32,
        pub num_channels: u32,
        pub num_samples: u32,
        pub sample_rate: f64,
        pub start_index: f64,
        pub channels: Vec<ChopChannel>,
    }

    #[derive(Debug, Default)]
    pub struct ChopOutputInfo {
        pub num_channels: u32,
        pub num_samples: u32,
        pub sample_rate: f64,
    }

    #[derive(Debug, Default)]
    pub struct ChopOutput {
        pub channels: Vec<ChopChannel>,
        pub num_channels: i32,
        pub num_samples: i32,
        pub sample_rate: i32,
    }

    extern "Rust" {
        fn on_reset(chop: &Chop);
        fn get_chop() -> Chop;
        fn get_output_info(info: &mut ChopOutputInfo, inputs: &ChopOperatorInputs) -> bool;
        fn chop_execute(output: &mut ChopOutput, inputs: &ChopOperatorInputs);
    }
}

fn on_reset(chop: &Chop) {
    println!("Reset!!");
}

fn get_chop() -> Chop {
    let mut c = Chop::default();
    let mut n = NumericParameter::default();
    n.name = "Speed".to_string();
    n.label = "Speed".to_string();
    n.page = "Custom".to_string();
    n.default_values[0] = 1.0;
    n.min_sliders[0] = -10.0;
    n.max_sliders[0] = 10.0;
    c.params.push(n);

    c.info.operator_type = "test1".to_string();
    c
}

fn get_output_info(info: &mut ChopOutputInfo, chop: &ChopOperatorInputs) -> bool {
    info.sample_rate = 60.0;
    info.num_samples = 60;
    info.num_channels = 1;
    true
}

fn chop_execute(output: &mut ChopOutput, inputs: &ChopOperatorInputs) {
    for i in 0..output.num_channels {
        let mut chan = ChopChannel::default();
        for j in 0..output.sample_rate {
            chan.data.push(j as f32 * 10.0);
        }
        output.channels.push(chan);
    }
    println!("{:?} {:?}", output, inputs);
}