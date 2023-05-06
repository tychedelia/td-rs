#include "CPlusPlus_Common.h"
#include <memory>

#ifndef TD_RS_RUSTBASE_H
#define TD_RS_RUSTBASE_H

struct RS_PrimitiveInfo {
    int32_t			numVertices;
    const int32_t*	pointIndices;
    PrimitiveType	type;
    int32_t			pointIndicesOffset;
};

std::unique_ptr<RS_PrimitiveInfo> getPrimitiveInfo(const OP_SOPInput* input, int32_t index) {
    auto info = input->getPrimitive(index);
    return std::make_unique<RS_PrimitiveInfo>(RS_PrimitiveInfo {
        .numVertices = info.numVertices,
        .pointIndices = info.pointIndices,
        .type = info.type,
        .pointIndicesOffset = info.pointIndicesOffset,
    });
}

void setString(OP_String *dest, const char *src) {
    dest->setString(src);
}

#endif //TD_RS_RUSTBASE_H
