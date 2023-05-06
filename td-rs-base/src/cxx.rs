use autocxx::prelude::*;
use std::pin::Pin;

include_cpp! {
    #include "CPlusPlus_Common.h"
    #include "RustBase.h"
    safety!(unsafe)
    generate!("OP_ParameterManager")
    generate!("OP_String")
    generate_pod!("OP_CHOPInput")
    generate!("OP_SOPInput")
    generate_pod!("OP_NumericParameter")
    generate_pod!("OP_StringParameter")
    generate!("OP_Inputs")
    generate_pod!("OP_InfoDATSize")
    generate_pod!("OP_InfoCHOPChan")

    // util fns
    generate!("setString")
}

pub use ffi::*;