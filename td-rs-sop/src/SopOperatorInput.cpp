#include "SopOperatorInput.h"

namespace td_rs_sop {
    SopOperatorInput::SopOperatorInput(const OP_Inputs *inputs) noexcept {
        this->inputs = inputs;
    }
}