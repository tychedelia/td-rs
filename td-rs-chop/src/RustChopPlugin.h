#include <memory>
#include "CHOP_CPlusPlusBase.h"
#include "CPlusPlus_Common.h"
#include <iostream>

#ifndef TD_RS_RUSTCHOP_H
#define TD_RS_RUSTCHOP_H

class RustChopPlugin : public CHOP_CPlusPlusBase {
public:
    virtual ~RustChopPlugin() {};

    void getGeneralInfo(CHOP_GeneralInfo* info, const OP_Inputs *inputs, void* reserved1) override {
        std::cout << "getGeneralInfo" << std::endl;
        this->getGeneralInfo(*info, *inputs);
    }

    virtual void getGeneralInfo(CHOP_GeneralInfo &info, const OP_Inputs &inputs) {}

    bool getOutputInfo(CHOP_OutputInfo* info, const OP_Inputs *inputs, void *reserved1) override {
        std::cout << "getOutputInfo" << std::endl;
        return this->getOutputInfo(*info, *inputs);
    }

    virtual bool getOutputInfo(CHOP_OutputInfo &info, const OP_Inputs &inputs) {
        return false;
    }

    void getChannelName(int32_t index, OP_String *name, const OP_Inputs *inputs, void* reserved1) override {
        std::cout << "getChannelName" << std::endl;
        this->getChannelName(index, *name, *inputs);
    }

    virtual void getChannelName(int32_t index, OP_String &name, const OP_Inputs &inputs) {}

    void execute(CHOP_Output *outputs, const OP_Inputs *inputs, void *reserved1) override {
        std::cout << "execute" << std::endl;
        this->execute(*outputs, *inputs);
    };

    virtual void execute(CHOP_Output &outputs, const OP_Inputs &inputs) {}

    virtual int32_t getNumInfoCHOPChans(void *reserved1) override {
        std::cout << "getNumInfoCHOPChans" << std::endl;
        return this->getNumInfoCHOPChans();
    }

    virtual int32_t getNumInfoCHOPChans() {
        return 0;
    }

    void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan* chan, void* reserved1) override {
        std::cout << "getInfoCHOPChan" << std::endl;
        this->getInfoCHOPChan(index, *chan);
    }

    virtual void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan &chan) {}

    bool getInfoDATSize(OP_InfoDATSize* infoSize, void *reserved1) override {
        std::cout << "getInfoDATSize" << std::endl;
        return this->getInfoDATSize(*infoSize);
    }

    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) {
        return false;
    }

    void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries* entries, void *reserved1) override {
        std::cout << "getInfoDATEntries" << std::endl;
        this->getInfoDATEntries(index, nEntries, *entries);
    }

    virtual void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries &entries) {}

    void getWarningString(OP_String *warning, void *reserved1) override {
        std::cout << "getWarningString" << std::endl;
        this->getWarningString(*warning);
    };

    virtual void getWarningString(OP_String &warning) {}

    void getErrorString(OP_String *error, void *reserved1) override {
        std::cout << "getErrorString" << std::endl;
        this->getErrorString(*error);
    };

    virtual void getErrorString(OP_String &error) {}

    void getInfoPopupString(OP_String *popup, void *reserved1) override {
        std::cout << "getInfoPopupString" << std::endl;
        this->getInfoPopupString(*popup);
    };

    virtual void getInfoPopupString(OP_String &popup) {}

    void setupParameters(OP_ParameterManager *manager, void *reserved1) override {
        std::cout << "setupParameters" << std::endl;
        this->setupParameters(*manager);
    };

    virtual void setupParameters(OP_ParameterManager &manager) {}

    void pulsePressed(const char *name, void *reserved1) override {
        std::cout << "pulsePressed" << std::endl;
        this->pulsePressed(name);
    };

    virtual void pulsePressed(const char *name) {}
};

class RustChopPluginWrapper : public RustChopPlugin {
public:
    virtual ~RustChopPluginWrapper() {};

    virtual void getGeneralInfo(CHOP_GeneralInfo &info, const OP_Inputs &inputs) = 0;
    virtual bool getOutputInfo(CHOP_OutputInfo &info, const OP_Inputs &inputs) = 0;
    virtual void getChannelName(int32_t index, OP_String &name, const OP_Inputs &inputs) = 0;
    virtual void execute(CHOP_Output &outputs, const OP_Inputs &inputs) = 0;
    virtual int32_t getNumInfoCHOPChans() = 0;
    virtual void getInfoCHOPChan(int32_t index, OP_InfoCHOPChan &chan) = 0;
    virtual bool getInfoDATSize(OP_InfoDATSize &infoSize) = 0;
    virtual void getInfoDATEntries(int32_t index, int32_t nEntries, OP_InfoDATEntries &entries) = 0;
    virtual void getWarningString(OP_String &warning) = 0;
    virtual void getErrorString(OP_String &error) = 0;
    virtual void getInfoPopupString(OP_String &popup) = 0;
    virtual void setupParameters(OP_ParameterManager &manager) = 0;
    virtual void pulsePressed(const char *name) = 0;
};

#endif //TD_RS_RUSTCHOP_H
