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
        target/aarch64-apple-darwin/cxxbridge/rust/cxx.h
        target/aarch64-apple-darwin/release/build/cxx-84d007bc7faa016c/out/flag_check.cpp
        target/aarch64-apple-darwin/release/build/link-cplusplus-0be262bd7c8e367d/out/dummy.cc
        target/cxxbridge/rust/cxx.h
        target/tests/trybuild/cxxbridge/rust/cxx.h
        td-rs-base/src/operator_input/OperatorInput.cpp
        td-rs-base/src/operator_input/OperatorInput.h
        td-rs-base/src/parameter_manager/ParameterManager.cpp
        td-rs-base/src/parameter_manager/ParameterManager.h
        td-rs-base/src/CPlusPlus_Common.h
        td-rs-chop/src/BoxDynChop.cpp
        td-rs-chop/src/BoxDynChop.h
        td-rs-chop/src/CHOP_CPlusPlusBase.h
        td-rs-chop/src/RustChopPlugin.cpp
        td-rs-chop/src/RustCHOP.h
        td-rs-sop/src/BoxDynSop.cpp
        td-rs-sop/src/BoxDynSop.h
        td-rs-sop/src/RustSOP.cpp
        td-rs-sop/src/RustSOP.h
        td-rs-sop/src/SOP_CPlusPlusBase.h
        td-rs-sop/src/SopInput.cpp
        td-rs-sop/src/SopInput.h
        td-rs-sop/src/SopOperatorInput.cpp
        td-rs-sop/src/SopOperatorInput.h
        td-rs-sop/src/SopOutput.cpp
        td-rs-sop/src/SopOutput.h
        td-rs-chop-autocxx/src/RustChopPlugin.h
        td-rs-chop-autocxx/src/RustChopPlugin.cpp)
