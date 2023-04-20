#include "CHOP_CPlusPlusBase.h"

#ifndef TD_RS_RUSTCHOP_H
#define TD_RS_RUSTCHOP_H
class RustChopPlugin : public CHOP_CPlusPlusBase {
public:
    virtual ~RustChopPlugin() = 0;
    virtual void setupParameters(OP_ParameterManager* manager, void* reserved1) = 0;
    void		execute(CHOP_Output* outputs,
                                const OP_Inputs* inputs,
                                void* reserved1) override = 0;
};
#endif //TD_RS_RUSTCHOP_H
