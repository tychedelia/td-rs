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

ChopParams BoxDynChop::getParams() noexcept {
    return chop_get_params(*this);
}

void BoxDynChop::onReset() noexcept {
    chop_on_reset(*this);
}

int32_t BoxDynChop::getNumInfoChopChans() noexcept {
    return chop_get_num_info_chop_chans(*this);
}

ChopInfoChan BoxDynChop::getInfoChopChan(int32_t index) noexcept {
    return chop_get_info_chop_chan(*this, index);
}

bool BoxDynChop::getOutputInfo(ChopOutputInfo* info, ChopOperatorInputs* inputs) noexcept {
    return chop_get_output_info(*this, *info, *inputs);
}

rust::String BoxDynChop::getChannelName(int32_t index, ChopOperatorInputs* inputs) noexcept {
    return chop_get_channel_name(*this, index, *inputs);
};

void BoxDynChop::execute(ChopOutput* output, ChopOperatorInputs* inputs) noexcept {
    chop_execute(*this, *output, *inputs);
}

