#include "SOP_CPlusPlusBase.h"
#include "SopInput.h"

namespace td_rs_sop {
    SopInput::SopInput(const OP_SOPInput *input) noexcept {
        this->input = input;
    }
}