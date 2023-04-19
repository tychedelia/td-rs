#include <td-rs-sop/src/cxx.rs.h>
#include <rust/cxx.h>

using namespace td_rs_base::ffi;

BoxDynSop::BoxDynSop(BoxDynSop &&other) noexcept : repr(other.repr) {
    other.repr = {0, 0};
}

BoxDynSop::~BoxDynSop() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
        dyn_sop_drop_in_place(this);
    }
}

void BoxDynSop::setupParams(ParameterManager* manager) noexcept {
    sop_setup_params(*this, *manager);
}

void BoxDynSop::onReset() noexcept {
    sop_on_reset(*this);
}

int32_t BoxDynSop::getNumInfoSopChans() noexcept {
    return sop_get_num_info_sop_chans(*this);
}

SopInfoChan BoxDynSop::getInfoSopChan(int32_t index) noexcept {
    return sop_get_info_sop_chan(*this, index);
}

bool BoxDynSop::getOutputInfo(SopOutputInfo* info, SopOperatorInput* sopInput) noexcept {
    return sop_get_output_info(*this, *info, *sopInput);
}

rust::String BoxDynSop::getChannelName(int32_t index, SopOperatorInput* sopInput) noexcept {
    return sop_get_channel_name(*this, index, *sopInput);
};

bool BoxDynSop::getInfoDatSize(SopInfoDatSize* size) noexcept {
    return sop_get_info_dat_size(*this, *size);
}

void BoxDynSop::getInfoDATEntries(int32_t index, int32_t nEntries, SopInfoDatEntries* entries) noexcept {
    return sop_get_info_dat_entries(*this, index, nEntries, *entries);
}

void BoxDynSop::execute(SopOutput* output, td_rs_base::ffi::OperatorInput* input, SopOperatorInput* sopInput) noexcept {
    sop_execute(*this, *output, *input, *sopInput);
}

SopGeneralInfo BoxDynSop::getGeneralInfo() noexcept {
    return sop_get_general_info(*this);
}

rust::String BoxDynSop::getWarningString() noexcept {
    return sop_get_warning(*this);
}

rust::String BoxDynSop::getErrorString() noexcept {
    return sop_get_error(*this);
}

rust::String BoxDynSop::getInfoString() noexcept {
    return sop_get_info(*this);
}
