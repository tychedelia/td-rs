use std::pin::Pin;
use crate::chop::Chop;
use crate::cxx::ffi::*;
use cxx::ExternType;
use crate::chop;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("td-rs-chop/src/ChopOutput.h");
        pub(crate) type ChopOutput;
        pub fn getNumChannels(&self) -> i32;
        pub fn getNumSamples(&self) -> i32;
        pub fn getSampleRate(&self) -> i32;
        pub fn getStartIndex(&self) -> usize;
        pub fn getChannelNames(&self) -> &[&str];
        pub fn getChannels(self: Pin<&mut ChopOutput>) -> &mut [&mut [f32]];
    }

    extern "Rust" {

    }
}