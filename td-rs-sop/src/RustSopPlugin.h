#include <memory>
#include "SOP_CPlusPlusBase.h"
#include "CPlusPlus_Common.h"
#include <iostream>

#ifndef TD_RS_RUSTSOP_H
#define TD_RS_RUSTSOP_H

using namespace TD;

class SopPlugin : public SOP_CPlusPlusBase {
public:
    virtual ~SopPlugin() {};

    void getGeneralInfo(SOP_GeneralInfo *info, const OP_Inputs *inputs, void *reserved1) {
        this->getGeneralInfo(*info, *inputs);
    }

    virtual void getGeneralInfo(SOP_GeneralInfo &info, const OP_Inputs &inputs) {}


    void execute(SOP_Output *outputs, const OP_Inputs *inputs, void *reserved1) {
        this->execute(*outputs, *inputs);
    }

    virtual void execute(SOP_Output &outputs, const OP_Inputs &inputs) {}

    void executeVBO(SOP_VBOOutput *output, const OP_Inputs *inputs, void *reserved1) {
        this->executeVBO(*output, *inputs);
    };

    virtual void executeVBO(SOP_VBOOutput &output, const OP_Inputs &inputs) {}


    int32_t getNumInfoCHOPChans(void *reserved1) {
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

    bool getInfoDATSize(OP_InfoDATSize *infoSize, void *reserved1) {
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
};

class RustSopPlugin : public SopPlugin {
public:
    virtual ~RustSopPlugin() {};

    virtual void getGeneralInfo(SOP_GeneralInfo &info, const OP_Inputs &inputs) = 0;

    virtual void execute(SOP_Output &outputs, const OP_Inputs &inputs) = 0;

    virtual void executeVBO(SOP_VBOOutput &output, const OP_Inputs &inputs) = 0;

    virtual int32_t getNumInfoCHOPChans() = 0;

    virtual void getInfoCHOPChan(int32_t index, OP_String &name, float &value) = 0;

    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) = 0;

    virtual void getInfoDATEntry(int32_t index, int32_t entryIndex, OP_String &entry) = 0;

    virtual void getWarningString(OP_String &warning) = 0;

    virtual void getErrorString(OP_String &error) = 0;

    virtual void getInfoPopupString(OP_String &popup) = 0;

    virtual void setupParameters(OP_ParameterManager &manager) = 0;

    virtual void pulsePressed(const char *name) = 0;
};

#endif //TD_RS_RUSTSOP_H
