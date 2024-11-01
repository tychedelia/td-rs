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
    generate_pod!("TD::OP_TOPInputDownloadOptions")
    generate!("TD::OP_SmartRef")
    generate_pod!("TD::OP_TextureDesc")
    generate_pod!("TD::OP_TexDim")

    // util fns
    generate!("setString")

    generate!("getDownloadDataSize")
    generate!("getDownloadData")
    generate!("getDownloadTextureDesc")
    generate!("releaseDownloadResult")

    // Custom ops
    generate!("TD::OP_CustomOPInstance")
    generate!("TD::CHOP_CPlusPlusBase")
    generate!("TD::DAT_CPlusPlusBase")
    generate!("TD::SOP_CPlusPlusBase")
    generate!("TD::TOP_CPlusPlusBase")
}

#[cfg(feature = "python")]
mod python {
    use autocxx::prelude::*;
    include_cpp! {
        #include "CPlusPlus_Common.h"
        #include "RustPy.h"
        name!(python_ffi)
        safety!(unsafe)
        extern_cpp_type!("TD::OP_CustomOPInfo", crate::cxx::OP_CustomOPInfo)
        pod!("TD::OP_CustomOPInfo")
        extern_cpp_type!("TD::PY_Context", crate::cxx::PY_Context)
        extern_cpp_type!("TD::PY_Struct", crate::cxx::PY_Struct)
        generate!("getPyContext")
        generate!("setPyInfo")
    }

    pub use python_ffi::getPyContext;
    pub use python_ffi::setPyInfo;
}

pub trait AsPlugin {
    type Plugin;
    fn as_plugin(&self) -> &Self::Plugin;
    fn as_plugin_mut(&mut self) -> std::pin::Pin<&mut Self::Plugin>;
}

pub use ffi::TD::*;
pub use ffi::*;
#[cfg(feature = "python")]
pub use python::*;
