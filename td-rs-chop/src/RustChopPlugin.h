#include <memory>
#include "CHOP_CPlusPlusBase.h"
#include "CPlusPlus_Common.h"

#ifndef TD_RS_RUSTCHOP_H
#define TD_RS_RUSTCHOP_H

class RustChopPlugin : public CHOP_CPlusPlusBase {
public:
    virtual ~RustChopPlugin() {};

    void getGeneralInfo(CHOP_GeneralInfo* info, const OP_Inputs *inputs, void* reserved1) override {
        this->getGeneralInfo(*info, *inputs);
    }

    virtual void getGeneralInfo(CHOP_GeneralInfo &info, const OP_Inputs &inputs) = 0;

    bool getOutputInfo(CHOP_OutputInfo* info, const OP_Inputs *inputs, void *reserved1) override {
        return this->getOutputInfo(*info, *inputs);
    }

    virtual bool getOutputInfo(CHOP_OutputInfo &info, const OP_Inputs &inputs) = 0;

    void getChannelName(int32_t index, OP_String *name, const OP_Inputs *inputs, void* reserved1) override {
        this->getChannelName(index, *name, *inputs);
    }

    virtual void getChannelName(int32_t index, OP_String &name, const OP_Inputs &inputs) = 0;


    void execute(CHOP_Output *outputs, const OP_Inputs *inputs, void *reserved1) override {
        this->execute(*outputs, *inputs);
    };

    virtual void execute(CHOP_Output &outputs, const OP_Inputs &inputs) = 0;

    int32_t getNumInfoCHOPChans(void *reserved1) override {
        return 0;
    }

    virtual int32_t getNumInfoCHOPChans() = 0;

    virtual void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan* chan, void* reserved1) override {
        this->getInfoCHOPChan(index, *chan);
    }

    virtual void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan &chan) = 0;

    virtual bool getInfoDATSize(OP_InfoDATSize* infoSize, void *reserved1) override {
        return this->getInfoDATSize(*infoSize);
    }

    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) = 0;

    virtual void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries* entries, void *reserved1) override {
        this->getInfoDATEntries(index, nEntries, *entries);
    }

    virtual void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries &entries) = 0;

    void getWarningString(OP_String *warning, void *reserved1) override {
        this->getWarningString(*warning);
    };

    virtual void getWarningString(OP_String &warning) = 0;

    void getErrorString(OP_String *error, void *reserved1) override {
        this->getErrorString(*error);
    };

    virtual void getErrorString(OP_String &error) = 0;

    void getInfoPopupString(OP_String *popup, void *reserved1) override {
        this->getInfoPopupString(*popup);
    };

    virtual void getInfoPopupString(OP_String &popup) = 0;

    void setupParameters(OP_ParameterManager *manager, void *reserved1) override {
        this->setupParameters(*manager);
    };

    virtual void setupParameters(OP_ParameterManager &manager) = 0;

    void pulsePressed(const char *name, void *reserved1) override {
        this->pulsePressed(name);
    };

    virtual void pulsePressed(const char *name) = 0;
};

#endif //TD_RS_RUSTCHOP_H
