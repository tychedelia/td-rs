#include "lib.rs.h"
#include "cxx.h"

BoxDynChop::BoxDynChop(BoxDynChop &&other) noexcept : repr(other.repr) {
    other.repr = {0, 0};
}

BoxDynChop::~BoxDynChop() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
        dyn_chop_drop_in_place(this);
    }
}

ChopParams BoxDynChop::get_params() noexcept {
    return chop_get_params(*this);
}

void BoxDynChop::on_reset() noexcept {
    chop_on_reset(*this);
}

bool BoxDynChop::get_output_info(ChopOutputInfo* info, ChopOperatorInputs* inputs) noexcept {
    return chop_get_output_info(*this, *info, *inputs);
}


void BoxDynChop::execute(ChopOutput* output, ChopOperatorInputs* inputs) noexcept {
    chop_execute(*this, *output, *inputs);
}

