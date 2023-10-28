use autocxx::prelude::*;

include_cpp! {
    #include "CPlusPlus_Common.h"
    #include "RustBase.h"
    safety!(unsafe)
    generate!("TD::OP_ParameterManager")
    generate!("TD::OP_String")
    generate_pod!("TD::OP_CustomOPInfo")
    generate_pod!("TD::OP_CHOPInput")
    generate!("TD::OP_SOPInput")
    generate_pod!("TD::OP_NumericParameter")
    generate_pod!("TD::OP_StringParameter")
    generate!("TD::OP_Inputs")
    generate_pod!("TD::OP_InfoDATSize")
    generate_pod!("TD::OP_InfoCHOPChan")
    generate_pod!("TD::Vector")
    generate_pod!("TD::Position")
    generate_pod!("TD::Color")
    generate_pod!("TD::TexCoord")
    generate_pod!("TD::BoundingBox")
    generate_pod!("TD::SOP_NormalInfo")
    generate_pod!("TD::SOP_ColorInfo")
    generate_pod!("TD::SOP_TextureInfo")
    generate_pod!("TD::SOP_CustomAttribData")
    generate_pod!("TD::SOP_PrimitiveInfo")
    generate_pod!("TD::OP_DATInput")
    generate_pod!("TD::OP_NodeInfo")
    generate!("TD::OP_Context")
    generate!("TD::OP_TOPInput")
    generate_pod!("TD::OP_TOPInputDownloadOptions")
    generate_pod!("TD::PY_Struct")
    generate_pod!("TD::PY_GetInfo")
    generate!("TD::PY_Context")

    // util fns
    generate!("setString")
    generate!("getPyContext")
    generate!("setPyInfo")
    generate!("getOpContext")
}

pub use ffi::TD::*;
pub use ffi::*;
