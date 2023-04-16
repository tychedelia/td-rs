#include "CHOP_CPlusPlusBase.h"
#include "ChopInput.h"

ChopInput::ChopInput(OP_CHOPInput* in) noexcept {
    input = in;
}

rust::Str ChopInput::getPath() const {
  return rust::Str(input->opPath);
}

uint32_t ChopInput::getId() const {
  return input->opId;
}

int32_t ChopInput::getNumChannels() const {
  return input->numChannels;
}

int32_t ChopInput::getNumSamples() const {
  return input->numSamples;
}

double ChopInput::getSampleRate() const {
  return input->sampleRate;
}

double ChopInput::getStartIndex() const {
  return input->startIndex;
}

rust::Slice<const rust::Slice<const float>> ChopInput::getChannels() const {
    auto channels = input->channelData;
    rust::Slice<const float> slices[getNumChannels()];
    for (auto i = 0; i < getNumChannels(); i++) {
        slices[i] = rust::Slice<const float> {channels[i], static_cast<size_t>(getNumSamples())};
    }

    return rust::Slice<const rust::Slice<const float>>(slices, getNumChannels());
}

rust::Slice<const rust::Str> ChopInput::getChannelNames() const {
      auto names = input->nameData;
      rust::Str channelNames[getNumChannels()];
      for (auto i = 0; i < getNumChannels(); i++) {
          channelNames[i] = rust::Str(names[i]);
      }
      return rust::Slice<const rust::Str>(channelNames, getNumChannels());
}

int64_t ChopInput::getTotalCooks() const {
  return input->totalCooks;
}
