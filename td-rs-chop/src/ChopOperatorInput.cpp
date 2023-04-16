#include "operator_input/OperatorInput.h"
#include "ChopInput.h"

using namespace td_rs_base::ffi;

class ChopOperatorInput : public OperatorInput {

public:
ChopOperatorInput(const OP_Inputs* inputs) noexcept;

virtual ChopInput getChopInput(int32_t index) const;
}