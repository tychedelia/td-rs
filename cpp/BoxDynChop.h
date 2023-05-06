#pragma once
#include <array>
#include <cstdint>
#include <type_traits>

// Forward declarations from lib.rs.h
struct ChopParams;
struct ChopOperatorInputs;
struct ChopOutputInfo;
struct ChopOutput;
struct OperatorInfo;

class BoxDynChop {
public:
    BoxDynChop(BoxDynChop &&) noexcept;
    ~BoxDynChop() noexcept;
    using IsRelocatable = std::true_type;

    void on_reset() noexcept;
    ChopParams get_params() noexcept;
    bool get_output_info(ChopOutputInfo* info, ChopOperatorInputs* inputs) noexcept;
    void execute(ChopOutput* output, ChopOperatorInputs* inputs) noexcept;

private:
    std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynChop = BoxDynChop*;