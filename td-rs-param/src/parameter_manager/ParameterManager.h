#pragma once
#include "../CPlusPlus_Common.h"
#include <rust/cxx.h>

namespace td_rs_param::ffi {

struct NumericParameter;
struct StringParameter;

class ParameterManager {
public:
    ParameterManager(OP_ParameterManager *mgr) noexcept;

    virtual void appendFloat(NumericParameter np) const;

    virtual void appendInt(NumericParameter np) const;

    virtual void appendXY(NumericParameter np) const;

    virtual void appendXYZ(NumericParameter np) const;

    virtual void appendUV(NumericParameter np) const;

    virtual void appendUVW(NumericParameter np) const;

    virtual void appendRGB(NumericParameter np) const;

    virtual void appendRGBA(NumericParameter np) const;

    virtual void appendToggle(NumericParameter np) const;

    virtual void appendPulse(NumericParameter np) const;

    virtual void appendString(StringParameter sp) const;

    virtual void appendFile(StringParameter sp) const;

    virtual void appendFolder(StringParameter sp) const;

    virtual void appendDAT(StringParameter sp) const;

    virtual void appendCHOP(StringParameter sp) const;

    virtual void appendTOP(StringParameter sp) const;

    virtual void appendObject(StringParameter sp) const;

    virtual void appendMenu(StringParameter sp, rust::Slice<const rust::Str> names, rust::Slice <const rust::Str> labels) const;

    virtual void appendStringMenu(StringParameter sp, rust::Slice <const rust::Str> names, rust::Slice <const rust::Str> labels) const;

    virtual void appendSOP(StringParameter sp) const;

    virtual void appendPython(StringParameter sp) const;

    virtual void appendOP(StringParameter sp) const;

    virtual void appendCOMP(StringParameter sp) const;

    virtual void appendMAT(StringParameter sp) const;

    virtual void appendPanelCOMP(StringParameter sp) const;

    virtual void appendHeader(StringParameter np) const;

    virtual void appendMomentary(NumericParameter np) const;

    virtual void appendWH(NumericParameter np) const;

private:
    OP_ParameterManager *manager;

    OP_NumericParameter mapNumeric(NumericParameter np) const;

    OP_StringParameter mapString(StringParameter sp) const;
};

}