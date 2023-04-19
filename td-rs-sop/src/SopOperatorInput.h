#pragma once
#include "operator_input/OperatorInput.h"
#include "SopInput.h"

using namespace td_rs_base::ffi;

namespace td_rs_sop {
    class SopOperatorInput {

    public:
        SopOperatorInput(const OP_Inputs *inputs) noexcept;

    private:
        const OP_Inputs *inputs;
    };
}