#pragma once
#include "../CPlusPlus_Common.h"
#include <rust/cxx.h>

namespace td_rs_base::ffi {

class OperatorInput {
public:
    OperatorInput(const OP_Inputs* inputs) noexcept;
    virtual int32_t		getNumInputs();
    virtual double		            getParDouble(rust::Str, int32_t index) const;
    virtual rust::Slice<const double>		getParDouble2(rust::Str) const;
    virtual rust::Slice<const double>		getParDouble3(rust::Str) const;
    virtual rust::Slice<const double>		getParDouble4(rust::Str) const;
    virtual int32_t		            getParInt(rust::Str name, int32_t index) const;
    virtual rust::Slice<const int32_t>	getParInt2(rust::Str name) const;
    virtual rust::Slice<const int32_t>	getParInt3(rust::Str name)const ;
    virtual rust::Slice<const int32_t>	getParInt4(rust::Str name) const;
    virtual rust::Str	            getParString(rust::Str name) const;

private:
    const OP_Inputs *inputs;
};

}
