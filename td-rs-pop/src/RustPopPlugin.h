#include <memory>
#include "POP_CPlusPlusBase.h"
#include "CPlusPlus_Common.h"

#ifndef TD_RS_RUSTPOP_H
#define TD_RS_RUSTPOP_H

using namespace TD;

OP_SmartRef<POP_Buffer> popCreateBuffer(POP_Context &context, uint64_t size,
                                        int32_t mode, int32_t usage);

void *popBufferData(OP_SmartRef<POP_Buffer> &buf);

uint64_t popBufferSize(const OP_SmartRef<POP_Buffer> &buf);

void popReleaseBuffer(OP_SmartRef<POP_Buffer> &buf);

void popSetAttribute(POP_Output &output, OP_SmartRef<POP_Buffer> *buf,
                     const char *name, uint32_t numComponents,
                     uint32_t numColumns, uint32_t arraySize, int32_t type,
                     int32_t qualifier, int32_t attribClass);

void popSetIndexBuffer(POP_Output &output, OP_SmartRef<POP_Buffer> *buf);

void popSetInfoBuffers(POP_Output &output, OP_SmartRef<POP_Buffer> *pointInfo,
                       OP_SmartRef<POP_Buffer> *topoInfo,
                       OP_SmartRef<POP_Buffer> *lineStripsInfo,
                       OP_SmartRef<POP_Buffer> *lineStripsPrimIndices,
                       OP_SmartRef<POP_Buffer> *gridInfo);

uint64_t popPointInfoSize();

uint64_t popTopologyInfoSize();

void popWritePointInfo(OP_SmartRef<POP_Buffer> &buf, uint32_t numPoints);

void popWriteTopologyInfo(OP_SmartRef<POP_Buffer> &buf,
                          uint32_t trianglesStartIndex, uint32_t trianglesCount,
                          uint32_t quadsStartIndex, uint32_t quadsCount,
                          uint32_t lineStripsStartIndex,
                          uint32_t lineStripsCount,
                          uint32_t lineStripsNumVertices,
                          uint32_t linesStartIndex, uint32_t linesCount,
                          uint32_t pointPrimitivesStartIndex,
                          uint32_t pointPrimitivesCount);

class PopPlugin : public POP_CPlusPlusBase {
public:
    virtual ~PopPlugin() {};

    void getGeneralInfo(POP_GeneralInfo *info, const OP_Inputs *inputs, void *reserved1) override {
        this->getGeneralInfo(*info, *inputs);
    }

    virtual void getGeneralInfo(POP_GeneralInfo &info, const OP_Inputs &inputs) {}

    void execute(POP_Output *output, const OP_Inputs *inputs, void *reserved1) override {
        this->execute(*output, *inputs);
    }

    virtual void execute(POP_Output &output, const OP_Inputs &inputs) {}

    int32_t getNumInfoCHOPChans(void *reserved1) override {
        return this->getNumInfoCHOPChans();
    }

    virtual int32_t getNumInfoCHOPChans() {
        return 0;
    }

    void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan *chan, void *reserved1) override {
        OP_String *name = chan->name;
        float v = 0.f;
        float *value = &v;
        this->getInfoCHOPChan(index, *name, *value);
        chan->name = name;
        chan->value = *value;
    }

    virtual void getInfoCHOPChan(int32_t index, OP_String &name, float &value) {}

    bool getInfoDATSize(OP_InfoDATSize *infoSize, void *reserved1) override {
        return this->getInfoDATSize(*infoSize);
    }

    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) {
        return false;
    }

    void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries *entries, void *reserved1) override {
        for (int i = 0; i < nEntries; i++) {
            auto entry = entries->values[i];
            this->getInfoDATEntry(index, i, *entry);
        }
    }

    virtual void getInfoDATEntry(int32_t index, int32_t entryIndex, OP_String &entry) {}

    void getWarningString(OP_String *warning, void *reserved1) override {
        this->getWarningString(*warning);
    };

    virtual void getWarningString(OP_String &warning) {}

    void getErrorString(OP_String *error, void *reserved1) override {
        this->getErrorString(*error);
    };

    virtual void getErrorString(OP_String &error) {}

    void getInfoPopupString(OP_String *popup, void *reserved1) override {
        this->getInfoPopupString(*popup);
    };

    virtual void getInfoPopupString(OP_String &popup) {}

    void setupParameters(OP_ParameterManager *manager, void *reserved1) override {
        this->setupParameters(*manager);
    };

    virtual void setupParameters(OP_ParameterManager &manager) {}

    void pulsePressed(const char *name, void *reserved1) override {
        this->pulsePressed(name);
    };

    virtual void pulsePressed(const char *name) {}

    void buildDynamicMenu(const OP_Inputs *inputs,
                          OP_BuildDynamicMenuInfo *info,
                          void *reserved1) override {
        this->buildDynamicMenu(*inputs, *info);
    }

    virtual void buildDynamicMenu(const OP_Inputs &inputs,
                                  OP_BuildDynamicMenuInfo &info) {}
};

class RustPopPlugin : public PopPlugin {
public:
    virtual ~RustPopPlugin() {};

    virtual void* inner() const = 0;

    virtual void* innerMut() = 0;

    virtual void getGeneralInfo(POP_GeneralInfo &info, const OP_Inputs &inputs) = 0;

    virtual void execute(POP_Output &output, const OP_Inputs &inputs) = 0;

    virtual int32_t getNumInfoCHOPChans() = 0;

    virtual void getInfoCHOPChan(int32_t index, OP_String &name, float &value) = 0;

    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) = 0;

    virtual void getInfoDATEntry(int32_t index, int32_t entryIndex, OP_String &entry) = 0;

    virtual void getWarningString(OP_String &warning) = 0;

    virtual void getErrorString(OP_String &error) = 0;

    virtual void getInfoPopupString(OP_String &popup) = 0;

    virtual void setupParameters(OP_ParameterManager &manager) = 0;

    virtual void pulsePressed(const char *name) = 0;

    virtual void buildDynamicMenu(const OP_Inputs &inputs,
                                  OP_BuildDynamicMenuInfo &info) = 0;
};

#endif //TD_RS_RUSTPOP_H
