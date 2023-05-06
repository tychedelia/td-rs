#![allow(unused)]

use td_rs_derive::*;
use td_rs_base::*;

#[derive(Params)]
struct TestParameter {
    #[label = "Hi"]
    float2: f32,
    float3: f64,
    int: i16,
    int2: u32,
    int3: i64,
    hi: String,
    // rgb: rgb::RGB<u8>,
}

fn main() {
    let mut param = TestParameter {
        // Initialize fields
        float2: 0.0,
        float3: 0.0,
        int: 0,
        int2: 0,
        int3: 0,
        hi: "".to_string(),
        // rgb: rgb::RGB::new(0,0,0),
    };
}
