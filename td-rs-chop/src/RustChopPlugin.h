#include <memory>
#include "CHOP_CPlusPlusBase.h"
#include "CPlusPlus_Common.h"

#ifndef TD_RS_RUSTCHOP_H
#define TD_RS_RUSTCHOP_H

class RustChopPlugin : public CHOP_CPlusPlusBase {
public:
    virtual ~RustChopPlugin() {};
// Raw pointer method calls std::unique_ptr method

//    // Overload execute method with std::unique_ptr
//    virtual void execute(std::unique_ptr<CHOP_Output> outputs,
//                         std::unique_ptr<const OP_Inputs> inputs) = 0;


    virtual void execute(CHOP_Output* outputs,
                         const OP_Inputs* inputs,
                         void* reserved1) override
    {
    }

    //
//    void setupParameters(OP_ParameterManager *manager, void *reserved1) override {
//        auto m = std::unique_ptr<OP_ParameterManager>(manager);
//        this->setupParameters(m);
//    };
//
//    virtual void setupParameters(std::unique_ptr<OP_ParameterManager> &manager) = 0;
//
//    void execute(CHOP_Output *outputs, const OP_Inputs *inputs, void *reserved1) {};
//
//    virtual void execute(std::unique_ptr<CHOP_Output> &outputs, std::unique_ptr<const OP_Inputs> &inputs) = 0;
};

#endif //TD_RS_RUSTCHOP_H
