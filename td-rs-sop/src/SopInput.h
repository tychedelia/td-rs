#pragma once
#include <rust/cxx.h>

namespace td_rs_sop {

    class SopInput {
    public:
        SopInput(const OP_SOPInput *input) noexcept;

    private:
        const OP_SOPInput *input;
    };

}
