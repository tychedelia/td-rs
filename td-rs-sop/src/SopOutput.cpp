#include "CHOP_CPlusPlusBase.h"
#include "ChopOutput.h"
#include "SopOutput.h"

#include <rust/cxx.h>

namespace td_rs_sop {

    SopOutput::SopOutput(SOP_Output *output) noexcept {
        this->output = output;
    }

    int32_t SopOutput::addPoint(const Position pos) {
        return 0;
    }

    bool SopOutput::addPoints(const rust::Slice<Position> positions) {
        return false;
    }

}