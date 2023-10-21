#![allow(unused)]

use td_rs_base::*;
use td_rs_derive_py::*;

enum TestEnum {
    Hi,
    Hello,
    Goodbye,
}

#[py_op]
struct TestParameter {
    float2: f32,
    float3: f64,
    int: i16,
    int2: u32,
    int3: i64,
    hi: String,
    menu: TestEnum,
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
        menu: TestEnum::Hi,
    };
}
