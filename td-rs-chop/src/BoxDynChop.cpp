#include <td-rs-chop/src/cxx.rs.h>
#include <rust/cxx.h>

using namespace td_rs_base::ffi;

BoxDynChop::BoxDynChop(BoxDynChop &&other) noexcept : repr(other.repr) {
    other.repr = {0, 0};
}

BoxDynChop::~BoxDynChop() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
        dyn_chop_drop_in_place(this);
    }
}

void BoxDynChop::setupParams(ParameterManager* manager) noexcept {
    chop_setup_params(*this, *manager);
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

bool BoxDynChop::getOutputInfo(ChopOutputInfo* info, ChopOperatorInput* chopInput) noexcept {
    return chop_get_output_info(*this, *info, *chopInput);
}

rust::String BoxDynChop::getChannelName(int32_t index, ChopOperatorInput* chopInput) noexcept {
    return chop_get_channel_name(*this, index, *chopInput);
};

bool BoxDynChop::getInfoDatSize(ChopInfoDatSize* size) noexcept {
    return chop_get_info_dat_size(*this, *size);
}

void BoxDynChop::getInfoDATEntries(int32_t index, int32_t nEntries, ChopInfoDatEntries* entries) noexcept {
    return chop_get_info_dat_entries(*this, index, nEntries, *entries);
}

void BoxDynChop::execute(ChopOutput* output, td_rs_base::ffi::OperatorInput* input, ChopOperatorInput* chopInput) noexcept {
    chop_execute(*this, *output, *input, *chopInput);
}

ChopGeneralInfo BoxDynChop::getGeneralInfo() noexcept {
    return chop_get_general_info(*this);
}

rust::String BoxDynChop::getWarningString() noexcept {
    return chop_get_warning(*this);
}

rust::String BoxDynChop::getErrorString() noexcept {
    return chop_get_error(*this);
}

rust::String BoxDynChop::getInfoString() noexcept {
    return chop_get_info(*this);
}
