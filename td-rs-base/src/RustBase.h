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

#endif // TD_RS_RUSTBASE_H
