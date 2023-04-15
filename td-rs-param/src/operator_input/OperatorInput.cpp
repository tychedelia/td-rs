#include "OperatorInput.h"
#include "CHOP_CPlusPlusBase.h"
#include <rust/cxx.h>

using namespace td_rs_param::ffi;

OperatorInput::OperatorInput(const OP_Inputs* inputs) noexcept {
    this->inputs = inputs;
}

int32_t OperatorInput::getNumInputs() {
    return inputs->getNumInputs();
}

double OperatorInput::getParDouble(const rust::Str name, int32_t index) const {
    return inputs->getParDouble(name.data(), index);
}

rust::Slice<const double> OperatorInput::getParDouble2(const rust::Str name) const {
    double v0, v1;
    bool success = inputs->getParDouble2(name.data(), v0, v1);
    if (success) {
        double data[] = {v0, v1};
        return rust::Slice<const double>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const double>();
    }
}

rust::Slice<const double> OperatorInput::getParDouble3(const rust::Str name) const {
    double v0, v1, v2;
    bool success = inputs->getParDouble3(name.data(), v0, v1, v2);
    if (success) {
        double data[] = {v0, v1, v2};
        return rust::Slice<const double>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const double>();
    }
}

rust::Slice<const double> OperatorInput::getParDouble4(const rust::Str name) const {
    double v0, v1, v2, v3;
    bool success = inputs->getParDouble4(name.data(), v0, v1, v2, v3);
    if (success) {
        double data[] = {v0, v1, v2, v3};
        return rust::Slice<const double>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const double>();
    }
}

int32_t OperatorInput::getParInt(rust::Str name, int32_t index) const {
    return inputs->getParInt(name.data(), index);
}

rust::Slice<const int32_t> OperatorInput::getParInt2(rust::Str name) const {
    int32_t v0, v1;
    bool success = inputs->getParInt2(name.data(), v0, v1);
    if (success) {
        int32_t data[] = {v0, v1};
        return rust::Slice<const int32_t>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const int32_t>();
    }
}

rust::Slice<const int32_t> OperatorInput::getParInt3(rust::Str name) const {
    int32_t v0, v1, v2;
    bool success = inputs->getParInt3(name.data(), v0, v1, v2);
    if (success) {
        int32_t data[] = {v0, v1, v2};
        return rust::Slice<const int32_t>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const int32_t>();
    }
}

rust::Slice<const int32_t> OperatorInput::getParInt4(rust::Str name) const {
    int32_t v0, v1, v2, v3;
    bool success = inputs->getParInt4(name.data(), v0, v1, v2, v3);
    if (success) {
        int32_t data[] = {v0, v1, v2, v3};
        return rust::Slice<const int32_t>(data, sizeof(data) / sizeof(data[0]));
    } else {
        return rust::Slice<const int32_t>();
    }
}

rust::Str OperatorInput::getParString(rust::Str name) const {
    const char* result = inputs->getParString(name.data());
    if (result == nullptr) {
        return rust::Str("");
    } else {
        return rust::Str(result);
    }
}