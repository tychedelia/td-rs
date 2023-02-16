#include "CHOP_CPlusPlusBase.h"
#include "ChopOutput.h"
#include <rust/cxx.h>

ChopOutput::ChopOutput(CHOP_Output* co) noexcept {
    output = co;
}

int32_t ChopOutput::getNumChannels() const {
    return output->numChannels;
}

int32_t ChopOutput::getNumSamples() const {
    return output->numSamples;
}

int32_t ChopOutput::getSampleRate() const {
    return output->sampleRate;
}

size_t ChopOutput::getStartIndex() const {
    return output->startIndex;
}

rust::Slice<const rust::Str> ChopOutput::getChannelNames() const {
    auto names = output->names;
    auto channelNames = rust::Slice<const rust::Str>();
    for (auto i = 0; i < getNumChannels(); i++) {
        rust::Str const name = rust::Str(names[i]);
//        channelNames[i] = name;
    }
    return channelNames;
}

rust::Slice<rust::Slice<float>> ChopOutput::getChannels() {
    auto channels = output->channels;
    rust::Slice<float> slices[getNumChannels()];
    for (auto i = 0; i < getNumChannels(); i++) {
        slices[i] = rust::Slice<float> {channels[i], static_cast<size_t>(getNumSamples())};
    }
    
    return rust::Slice<rust::Slice<float>>(slices, getNumChannels());
}
