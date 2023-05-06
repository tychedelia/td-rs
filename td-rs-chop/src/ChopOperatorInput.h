#pragma once
#include "operator_input/OperatorInput.h"
#include "ChopInput.h"

using namespace td_rs_base::ffi;

class ChopOperatorInput {
    
public:
    ChopOperatorInput(const OP_Inputs* inputs) noexcept;
    virtual std::unique_ptr<ChopInput> getInput(std::size_t index) const;
private:
    const OP_Inputs *inputs;
};
