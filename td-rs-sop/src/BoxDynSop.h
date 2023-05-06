#pragma once
#include "SopOutput.h"
#include "SopOperatorInput.h"
#include "parameter_manager/ParameterManager.h"
#include <array>
#include <cstdint>
#include <type_traits>
#include <rust/cxx.h>

// Forward declarations from cxx.rs.h
//struct SopOutputInfo;
//struct OperatorInfo;
//struct SopInfoChan;
//struct SopInfoDatSize;
//struct SopInfoDatEntries;
//struct SopGeneralInfo;

class BoxDynSop {
public:
    BoxDynSop(BoxDynSop &&) noexcept;
    ~BoxDynSop() noexcept;

    void setupParams(td_rs_base::ffi::ParameterManager* manager) noexcept;
    void execute(SopOutput* output, td_rs_base::ffi::OperatorInput* input, SopOperatorInput* sopInput) noexcept;
    void executeVBO(SopOutput* output, td_rs_base::ffi::OperatorInput* input, SopOperatorInput* sopInput) noexcept;
    SopGeneralInfo getGeneralInfo() noexcept;
    rust::String getWarningString() noexcept;

private:
    std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynSop = BoxDynSop*;