#include "ChopOperatorInput.h"

ChopOperatorInput::ChopOperatorInput(const OP_Inputs* inputs) noexcept {
    this->inputs = inputs;
}

std::unique_ptr<ChopInput> ChopOperatorInput::getInput(std::size_t index) const {
    auto in = inputs->getInputCHOP(index);
    return std::make_unique<ChopInput>(ChopInput(in));
}
