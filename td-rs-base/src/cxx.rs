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
    generate_pod!("Vector")
    generate_pod!("Position")
    generate_pod!("Color")
    generate_pod!("TexCoord")
    generate_pod!("BoundingBox")
    generate_pod!("SOP_NormalInfo")
    generate_pod!("SOP_ColorInfo")
    generate_pod!("SOP_TextureInfo")

    // util fns
    generate!("setString")
}

pub use ffi::*;