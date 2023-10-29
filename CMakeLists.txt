cmake_minimum_required(VERSION 3.25)
project(td_rs)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)
include_directories(target/aarch64-apple-darwin/cxxbridge/rust)
include_directories(target/aarch64-apple-darwin/cxxbridge/td-rs-base/src)
include_directories(target/aarch64-apple-darwin/cxxbridge/td-rs-chop/src)
include_directories(target/aarch64-apple-darwin/cxxbridge/td-rs-param/src)
include_directories(target/cxxbridge/rust)
include_directories(target/cxxbridge/td-rs-base/src)
include_directories(target/cxxbridge/td-rs-chop/src)
include_directories(target/cxxbridge/td-rs-param/src)
include_directories(td-rs-base/src)
include_directories(td-rs-base/src/operator_input)
include_directories(td-rs-base/src/parameter_manager)
include_directories(td-rs-chop/src)
include_directories(td-rs-sop/src)

add_executable(td_rs
        td-rs-base/src/CPlusPlus_Common.h
        td-rs-chop/src/CHOP_CPlusPlusBase.h
        td-rs-chop/src/RustChopPlugin.cpp
        td-rs-chop/src/RustChopPlugin.h
        td-rs-base/src/RustBase.h
        td-rs-sop/src/RustSopPlugin.h
        td-rs-top/src/RustTopPlugin.cpp
        td-rs-top/src/TOP_CPlusPlusBase.h
        td-rs-top/src/RustTopPlugin.h td-rs-dat/src/RustDatPlugin.cpp td-rs-sop/src/RustSopPlugin.cpp td-rs-base/src/RustPy.h)
