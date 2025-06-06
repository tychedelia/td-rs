#include "RustTopPlugin.h"
#include "CPlusPlus_Common.h"
#include <iostream>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif


extern "C" {

RustTopPlugin *top_new(const OP_NodeInfo &info, TOP_Context &context);
TOP_ExecuteMode top_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillTOPPluginInfo(TOP_PluginInfo *info) {
  info->apiVersion = TOPCPlusPlusAPIVersion;
  auto opInfo = &info->customOPInfo;
  auto mode = top_get_plugin_info_impl(*opInfo);
  info->executeMode = mode;
#ifdef PYTHON_ENABLED
    opInfo->pythonVersion->setString(PY_VERSION);
#endif
}

DLLEXPORT
TOP_CPlusPlusBase *CreateTOPInstance(const OP_NodeInfo *info, TOP_Context *context) {
  return top_new(*info, *context);
}

DLLEXPORT
void DestroyTOPInstance(TOP_CPlusPlusBase *instance) {
  delete (RustTopPlugin *)instance;
}
}

void* getBufferData(TD::OP_SmartRef<TD::TOP_Buffer> &buffer) {
    void* data = buffer->data;
    return data;
}

uint64_t getBufferSize(const TD::OP_SmartRef<TD::TOP_Buffer> &buffer) {
    uint64_t size = buffer->size;
    return size;
}

TD::TOP_BufferFlags getBufferFlags(const TD::OP_SmartRef<TD::TOP_Buffer> &buffer) {
    TD::TOP_BufferFlags flags = buffer->flags;
    return flags;
}

void releaseBuffer(TD::OP_SmartRef<TD::TOP_Buffer> &buffer) {
    // buffer may have been moved, so we need to check if it's valid
    if (buffer) {
        buffer.release();
    }
}

// CUDA helper functions (TOP-specific only)

TD::OP_TextureDesc getCUDAOutputInfoTextureDesc(const TD::TOP_CUDAOutputInfo &info) {
    return info.textureDesc;
}

void* getCUDAOutputInfoStream(const TD::TOP_CUDAOutputInfo &info) {
    return info.stream;
}

uint32_t getCUDAOutputInfoColorBufferIndex(const TD::TOP_CUDAOutputInfo &info) {
    return info.colorBufferIndex;
}

// Helper to create TOP_CUDAOutputInfo from primitive parameters
TD::TOP_CUDAOutputInfo createCUDAOutputInfo(void* stream, 
                                           uint32_t width, uint32_t height, uint32_t depth,
                                           int32_t texDim, int32_t pixelFormat,
                                           float aspectX, float aspectY,
                                           uint32_t colorBufferIndex) {
    TD::TOP_CUDAOutputInfo info;
    info.stream = static_cast<cudaStream_t>(stream);
    
    info.textureDesc.width = width;
    info.textureDesc.height = height;
    info.textureDesc.depth = depth;
    info.textureDesc.texDim = static_cast<TD::OP_TexDim>(texDim);
    info.textureDesc.pixelFormat = static_cast<TD::OP_PixelFormat>(pixelFormat);
    info.textureDesc.aspectX = aspectX;
    info.textureDesc.aspectY = aspectY;
    
    info.colorBufferIndex = colorBufferIndex;
    
    return info;
}


// Helper for CUDA context operations
bool beginCUDAOperations(TD::TOP_Context* context) {
    if (!context) return false;
    return context->beginCUDAOperations(nullptr);
}

void endCUDAOperations(TD::TOP_Context* context) {
    if (context) {
        context->endCUDAOperations(nullptr);
    }
}

