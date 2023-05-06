use crate::ffi::*;

#[cxx::bridge]
mod ffi {
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
    }

    #[derive(Debug)]
    pub struct ChopInput {
        pub data: Vec<f32>,
    }

    #[derive(Debug)]
    pub struct ChopOperatorInputs {
        pub path: String,
        pub id: u32,
        pub num_channels: u32,
        pub num_samples: u32,
        pub sample_rate: f64,
        pub start_index: f64,
        pub inputs: Vec<ChopInput>,
    }

    extern "Rust" {
        fn get_chop() -> Chop;
        fn chop_execute(chop: ChopOperatorInputs);
    }
}

fn get_chop() -> Chop {
    let mut c = Chop::default();
    c.info.operator_type = "test1".to_string();
    c
}

fn chop_execute(chop: ChopOperatorInputs) {
    println!("{:?}", chop);
}