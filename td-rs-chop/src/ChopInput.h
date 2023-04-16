#pragma once
#include <rust/cxx.h>

class ChopInput {
public:
  ChopInput(const OP_CHOPInput* input) noexcept;

  rust::Str getPath() const;
  uint32_t getId() const;

  int32_t getNumChannels() const;
  int32_t getNumSamples() const;
  double getSampleRate() const;
  double getStartIndex() const;

  rust::Slice<const rust::Slice<const float>> getChannels() const;
  rust::Slice<const rust::Str> getChannelNames() const;
  int64_t getTotalCooks() const;

private:
  const OP_CHOPInput* input;
};
