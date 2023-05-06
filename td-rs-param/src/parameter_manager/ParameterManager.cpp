#include "ParameterManager.h"
#include <td-rs-param/src/cxx.rs.h>
#include <rust/cxx.h>

using namespace td_rs_param::ffi;

ParameterManager::ParameterManager(OP_ParameterManager *mgr) noexcept {
    manager = mgr;
}

void ParameterManager::appendFloat(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendFloat(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendInt(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendInt(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendXY(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendXY(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendXYZ(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendXYZ(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendUV(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendUV(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendUVW(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendUVW(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendRGB(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendRGB(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendRGBA(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendRGBA(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendToggle(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendToggle(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendPulse(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendPulse(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendString(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendString(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendFile(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendFile(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendFolder(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendFolder(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendDAT(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendDAT(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendCHOP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendCHOP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendTOP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendTOP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendObject(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendObject(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendMenu(StringParameter sp, rust::Slice<const rust::Str> names,
                                  rust::Slice<const rust::Str> labels) const {
}

void ParameterManager::appendStringMenu(StringParameter sp, rust::Slice<const rust::Str> names,
                                        rust::Slice<const rust::Str> labels) const {

}

void ParameterManager::appendSOP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendSOP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendPython(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendPython(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendOP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendOP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendCOMP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendCOMP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendMAT(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendMAT(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendPanelCOMP(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendPanelCOMP(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendHeader(StringParameter sp) const {
    OP_ParAppendResult res = manager->appendHeader(this->mapString(sp));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendMomentary(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendMomentary(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

void ParameterManager::appendWH(NumericParameter np) const {
    OP_ParAppendResult res = manager->appendWH(this->mapNumeric(np));
    assert(res == OP_ParAppendResult::Success);
}

OP_NumericParameter ParameterManager::mapNumeric(NumericParameter np) const {
    OP_NumericParameter param;

    param.name = np.name.c_str();
    param.label = np.label.c_str();
    param.page = np.page.c_str();
    std::copy(std::begin(np.default_values), std::end(np.default_values), std::begin(param.defaultValues));
    std::copy(std::begin(np.max_values), std::end(np.max_values), std::begin(param.maxValues));
    std::copy(std::begin(np.min_values), std::end(np.min_values), std::begin(param.minValues));
    std::copy(std::begin(np.max_sliders), std::end(np.max_sliders), std::begin(param.maxSliders));
    std::copy(std::begin(np.min_sliders), std::end(np.min_sliders), std::begin(param.minSliders));
    std::copy(std::begin(np.clamp_maxes), std::end(np.clamp_maxes), std::begin(param.clampMaxes));
    std::copy(std::begin(np.clamp_mins), std::end(np.clamp_mins), std::begin(param.clampMins));

    return param;
}

OP_StringParameter ParameterManager::mapString(StringParameter sp) const {
    OP_StringParameter param;

    param.name = sp.name.c_str();
    param.label = sp.label.c_str();
    param.page = sp.page.c_str();
    param.defaultValue = sp.default_value.c_str();
    return param;
}
