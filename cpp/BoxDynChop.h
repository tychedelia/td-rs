#pragma once
#include <array>
#include <cstdint>
#include <type_traits>
#include "cxx.h"

// Forward declarations from lib.rs.h
struct ChopParams;
struct ChopOperatorInputs;
struct ChopOutputInfo;
struct ChopOutput;
struct OperatorInfo;
struct ChopInfoChan;

class BoxDynChop {
public:
    BoxDynChop(BoxDynChop &&) noexcept;
    ~BoxDynChop() noexcept;
    using IsRelocatable = std::true_type;

    void onReset() noexcept;
    ChopParams getParams() noexcept;
    bool getOutputInfo(ChopOutputInfo* info, ChopOperatorInputs* inputs) noexcept;
    int32_t getNumInfoChopChans() noexcept;
    ChopInfoChan getInfoChopChan(int32_t index) noexcept;
    rust::String getChannelName(int32_t index, ChopOperatorInputs* inputs) noexcept;
    void execute(ChopOutput* output, ChopOperatorInputs* inputs) noexcept;

private:
    std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynChop = BoxDynChop*;