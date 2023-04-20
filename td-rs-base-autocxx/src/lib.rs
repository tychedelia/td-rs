use autocxx::prelude::*;
use std::pin::Pin;

include_cpp! {
    #include "CPlusPlus_Common.h"
    safety!(unsafe)
    generate!("OP_String")
    generate_pod!("OP_CustomOPInfo")
    generate!("OP_CHOPInput")
    generate!("OP_ParameterManager")
    generate_pod!("OP_InfoDATEntries")
    generate_pod!("OP_NumericParameter")
    generate_pod!("OP_StringParameter")
}

pub use ffi::*;