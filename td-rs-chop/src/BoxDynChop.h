#pragma once
#include "ChopOutput.h"
#include "parameter_manager/ParameterManager.h"
#include "operator_input/OperatorInput.h"
#include <array>
#include <cstdint>
#include <type_traits>
#include <rust/cxx.h>

// Forward declarations from cxx.rs.h
struct ChopOutputInfo;
struct OperatorInfo;
struct ChopInfoChan;
struct ChopInfoDatSize;
struct ChopInfoDatEntries;
struct ChopGeneralInfo;

class BoxDynChop {
public:
    BoxDynChop(BoxDynChop &&) noexcept;
    ~BoxDynChop() noexcept;
    using IsRelocatable = std::true_type;

    void onReset() noexcept;
    void setupParams(td_rs_param::ffi::ParameterManager* manager) noexcept;
    bool getOutputInfo(ChopOutputInfo* info, td_rs_param::ffi::OperatorInput* input) noexcept;
    int32_t getNumInfoChopChans() noexcept;
    ChopInfoChan getInfoChopChan(int32_t index) noexcept;
    rust::String getChannelName(int32_t index, td_rs_param::ffi::OperatorInput* input) noexcept;
    bool getInfoDatSize(ChopInfoDatSize* size) noexcept;
    void getInfoDATEntries(int32_t index, int32_t nEntries, ChopInfoDatEntries* entries) noexcept;
    void execute(ChopOutput* output, td_rs_param::ffi::OperatorInput* input) noexcept;
    ChopGeneralInfo getGeneralInfo() noexcept;
    rust::String getWarningString() noexcept;
    rust::String getErrorString() noexcept;
    rust::String getInfoString() noexcept;

private:
    std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynChop = BoxDynChop*;