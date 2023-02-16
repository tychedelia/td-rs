#pragma once
#include "CHOP_CPlusPlusBase.h"
#include <rust/cxx.h>

class ChopOutput {
public:
    ChopOutput(CHOP_Output* co) noexcept;

    virtual int32_t  getNumChannels() const;
    virtual int32_t  getNumSamples() const;
    virtual int32_t  getSampleRate() const;
    virtual size_t   getStartIndex() const;

    virtual rust::Slice<const rust::Str> getChannelNames() const;
    virtual rust::Slice<rust::Slice<float>> getChannels();

private:
    CHOP_Output* output;
};
