#![allow(unused)]

use td_rs_derive::Params;
use td_rs_chop::chop::{ParameterManager, ChopParams};
use td_rs_chop::cxx::ffi::OperatorInput;

#[derive(Params)]
struct TestParameter {
    foo: f32
}

fn main() {
    let mut param = TestParameter {
        // Initialize fields
        foo: 0.0,
    };
}
