use autocxx::prelude::*;
use std::pin::Pin;

include_cpp! {
    #include "CPlusPlus_Common.h"
    safety!(unsafe)
    generate!("OP_ParameterManager")
    generate!("OP_String")
    generate!("OP_CHOPInput")
    generate_pod!("OP_NumericParameter")
    generate_pod!("OP_StringParameter")
    generate!("OP_Inputs")
    generate_pod!("OP_InfoDATSize")
    generate_pod!("OP_InfoCHOPChan")
    generate_pod!("OP_CustomOPInfo")
}

pub use ffi::*;