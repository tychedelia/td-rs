#include "CPlusPlus_Common.h"
#include "RustPopPlugin.h"
#include <cstring>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif

extern "C" {

RustPopPlugin *pop_new(const OP_NodeInfo &info, POP_Context &context);
void pop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillPOPPluginInfo(POP_PluginInfo *info) {
    info->apiVersion = POPCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    pop_get_plugin_info_impl(*opInfo);
#ifdef PYTHON_ENABLED
    opInfo->pythonVersion->setString(PY_VERSION);
#endif
}

DLLEXPORT
POP_CPlusPlusBase *CreatePOPInstance(const OP_NodeInfo *info, POP_Context *context) {
    return pop_new(*info, *context);
}

DLLEXPORT
void DestroyPOPInstance(POP_CPlusPlusBase *instance) {
    delete (RustPopPlugin *) instance;
}

}

OP_SmartRef<POP_Buffer> popCreateBuffer(POP_Context &context, uint64_t size,
                                        int32_t mode, int32_t usage) {
    POP_BufferInfo info;
    info.size = size;
    info.mode = static_cast<POP_BufferMode>(mode);
    info.usage = static_cast<POP_BufferUsage>(usage);
    info.location = POP_BufferLocation::CPU;
    return context.createBuffer(info, nullptr);
}

void *popBufferData(OP_SmartRef<POP_Buffer> &buf) {
    if (!buf) {
        return nullptr;
    }
    return buf->getData(nullptr);
}

uint64_t popBufferSize(const OP_SmartRef<POP_Buffer> &buf) {
    if (!buf) {
        return 0;
    }
    return buf->info.size;
}

uint64_t popPointInfoSize() { return sizeof(POP_PointInfo); }

uint64_t popTopologyInfoSize() { return sizeof(POP_TopologyInfo); }

void popReleaseBuffer(OP_SmartRef<POP_Buffer> &buf) {
    if (buf) {
        buf.release();
    }
}

void popSetAttribute(POP_Output &output, OP_SmartRef<POP_Buffer> *buf,
                     const char *name, uint32_t numComponents,
                     uint32_t numColumns, uint32_t arraySize, int32_t type,
                     int32_t qualifier, int32_t attribClass) {
    POP_AttributeInfo info;
    info.name = name;
    info.numComponents = numComponents;
    info.numColumns = numColumns;
    info.arraySize = arraySize;
    info.type = static_cast<POP_AttributeType>(type);
    info.qualifier = static_cast<POP_AttributeQualifier>(qualifier);
    info.attribClass = static_cast<POP_AttributeClass>(attribClass);
    POP_SetBufferInfo sinfo;
    output.setAttribute(buf, info, sinfo, nullptr);
    delete buf;
}

void popSetIndexBuffer(POP_Output &output, OP_SmartRef<POP_Buffer> *buf) {
    POP_IndexBufferInfo info;
    info.type = POP_IndexType::UInt32;
    POP_SetBufferInfo sinfo;
    output.setIndexBuffer(buf, info, sinfo, nullptr);
    delete buf;
}

void popSetInfoBuffers(POP_Output &output, OP_SmartRef<POP_Buffer> *pointInfo,
                       OP_SmartRef<POP_Buffer> *topoInfo,
                       OP_SmartRef<POP_Buffer> *lineStripsInfo,
                       OP_SmartRef<POP_Buffer> *lineStripsPrimIndices,
                       OP_SmartRef<POP_Buffer> *gridInfo) {
    POP_InfoBuffers bufs;
    if (pointInfo) {
        bufs.pointInfo = std::move(*pointInfo);
        delete pointInfo;
    }
    if (topoInfo) {
        bufs.topoInfo = std::move(*topoInfo);
        delete topoInfo;
    }
    if (lineStripsInfo) {
        bufs.lineStripsInfo = std::move(*lineStripsInfo);
        delete lineStripsInfo;
    }
    if (lineStripsPrimIndices) {
        bufs.lineStripsPrimIndices = std::move(*lineStripsPrimIndices);
        delete lineStripsPrimIndices;
    }
    if (gridInfo) {
        bufs.gridInfo = std::move(*gridInfo);
        delete gridInfo;
    }
    POP_SetBufferInfo sinfo;
    output.setInfoBuffers(&bufs, sinfo, nullptr);
}

void popWritePointInfo(OP_SmartRef<POP_Buffer> &buf, uint32_t numPoints) {
    auto *info = static_cast<POP_PointInfo *>(buf->getData(nullptr));
    *info = POP_PointInfo();
    info->numPoints = numPoints;
}

void popWriteTopologyInfo(OP_SmartRef<POP_Buffer> &buf,
                          uint32_t trianglesStartIndex, uint32_t trianglesCount,
                          uint32_t quadsStartIndex, uint32_t quadsCount,
                          uint32_t lineStripsStartIndex,
                          uint32_t lineStripsCount,
                          uint32_t lineStripsNumVertices,
                          uint32_t linesStartIndex, uint32_t linesCount,
                          uint32_t pointPrimitivesStartIndex,
                          uint32_t pointPrimitivesCount) {
    auto *info = static_cast<POP_TopologyInfo *>(buf->getData(nullptr));
    *info = POP_TopologyInfo();
    info->trianglesStartIndex = trianglesStartIndex;
    info->trianglesCount = trianglesCount;
    info->quadsStartIndex = quadsStartIndex;
    info->quadsCount = quadsCount;
    info->lineStripsStartIndex = lineStripsStartIndex;
    info->lineStripsCount = lineStripsCount;
    info->lineStripsNumVertices = lineStripsNumVertices;
    info->linesStartIndex = linesStartIndex;
    info->linesCount = linesCount;
    info->pointPrimitivesStartIndex = pointPrimitivesStartIndex;
    info->pointPrimitivesCount = pointPrimitivesCount;
}
