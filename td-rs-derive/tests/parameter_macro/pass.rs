#![allow(unused)]

use td_rs_derive::*;
use td_rs_base::*;

#[derive(Param)]
enum TestEnum {
    Hi,
    Hello,
    Goodbye,
}

#[derive(Params)]
struct TestParameter {
    #[param(label = "Hi")]
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

    assert_eq!(TestEnum::names(), [String::from("Hi"), String::from("Hello"), String::from("Goodbye")]);
}
