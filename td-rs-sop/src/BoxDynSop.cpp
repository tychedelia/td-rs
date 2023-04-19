#include <td-rs-sop/src/cxx.rs.h>
#include <rust/cxx.h>
#include "BoxDynSop.h"

using namespace td_rs_base::ffi;

namespace td_rs_sop {

    BoxDynSop::BoxDynSop(BoxDynSop &&other) noexcept: repr(other.repr) {
        other.repr = {0, 0};
    }

    BoxDynSop::~BoxDynSop() noexcept {
        if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
            dyn_sop_drop_in_place(this);
        }
    }

    void BoxDynSop::setupParams(td_rs_base::ffi::ParameterManager *manager) noexcept {
        sop_setup_params(*this, *manager);
    }

    void
    BoxDynSop::execute(SopOutput *output, td_rs_base::ffi::OperatorInput *input, SopOperatorInput *sopInput) noexcept {

    }

    void BoxDynSop::executeVBO(SopOutput *output, td_rs_base::ffi::OperatorInput *input,
                               SopOperatorInput *sopInput) noexcept {

    }

    SopGeneralInfo BoxDynSop::getGeneralInfo() noexcept {
        return sop_get_general_info(*this);
    }

    rust::String BoxDynSop::getWarningString() noexcept {
        return sop_get_warning(*this);
    }
}