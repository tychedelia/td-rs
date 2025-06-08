#include "CPlusPlus_Common.h"
#include <memory>
#include <vector>
#include <iostream>

#ifndef TD_RS_RUSTBASE_H
#define TD_RS_RUSTBASE_H

void setString(TD::OP_String *dest, const char *src) { dest->setString(src); }

uint64_t getDownloadDataSize(TD::OP_SmartRef<TD::OP_TOPDownloadResult> &result) {
    uint64_t size = result->size;
    return size;
}

void* getDownloadData(TD::OP_SmartRef<TD::OP_TOPDownloadResult> &result) {
    void* data = result->getData();
    return data;
}

TD::OP_TextureDesc getDownloadTextureDesc(TD::OP_SmartRef<TD::OP_TOPDownloadResult> &result) {
    TD::OP_TextureDesc desc = result->textureDesc;
    return desc;
}

void releaseDownloadResult(TD::OP_SmartRef<TD::OP_TOPDownloadResult> &result) {
    result.release();
}

const char* getBuildDynamicMenuInfoNames(TD::OP_BuildDynamicMenuInfo &info) {
    return info.name;
}

// CUDA helper functions for base types
TD::OP_TextureDesc getCUDAArrayInfoTextureDesc(const TD::OP_CUDAArrayInfo &info) {
    return info.textureDesc;
}

void* getCUDAArrayInfoArray(const TD::OP_CUDAArrayInfo &info) {
    return info.cudaArray;
}

void* getCUDAAcquireInfoStream(const TD::OP_CUDAAcquireInfo &info) {
    return info.stream;
}

TD::OP_CUDAAcquireInfo createCUDAAcquireInfo(void* stream) {
    TD::OP_CUDAAcquireInfo info;
    info.stream = static_cast<cudaStream_t>(stream);
    return info;
}

// Helper function to get texture descriptor from TOP input
TD::OP_TextureDesc getTOPInputTextureDesc(const TD::OP_TOPInput &input) {
    return input.textureDesc;
}

#endif // TD_RS_RUSTBASE_H
