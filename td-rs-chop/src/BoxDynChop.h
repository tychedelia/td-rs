#pragma once
#include "ChopOutput.h"
#include "ParameterManager.h"
#include "OperatorInput.h"
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
    void setupParams(ParameterManager* manager) noexcept;
    bool getOutputInfo(ChopOutputInfo* info, OperatorInput* input) noexcept;
    int32_t getNumInfoChopChans() noexcept;
    ChopInfoChan getInfoChopChan(int32_t index) noexcept;
    rust::String getChannelName(int32_t index, OperatorInput* input) noexcept;
    bool getInfoDatSize(ChopInfoDatSize* size) noexcept;
    void getInfoDATEntries(int32_t index, int32_t nEntries, ChopInfoDatEntries* entries) noexcept;
    void execute(ChopOutput* output, OperatorInput* input) noexcept;
    ChopGeneralInfo getGeneralInfo() noexcept;
    rust::String getWarningString() noexcept;
    rust::String getErrorString() noexcept;
    rust::String getInfoString() noexcept;

private:
    std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynChop = BoxDynChop*;