#include "RustSOP.h"
#include "BoxDynSop.h"
#include "SopInput.h"
#include "SopOutput.h"
#include "SopOperatorInput.h"
#include "ParameterManager.h"
#include <td-rs-sop/src/cxx.rs.h>
#include <rust/cxx.h>

extern "C" {

DLLEXPORT
void FillSOPPluginInfo(SOP_PluginInfo *info) {
  td_rs_sop::OperatorInfo sopInfo = sop_get_operator_info();
  info->apiVersion = SOPCPlusPlusAPIVersion;
  info->customOPInfo.opType->setString(sopInfo.operator_type.c_str());
  info->customOPInfo.opLabel->setString(sopInfo.operator_label.c_str());
  info->customOPInfo.authorName->setString(sopInfo.author_name.c_str());
  info->customOPInfo.authorEmail->setString(sopInfo.author_email.c_str());
  info->customOPInfo.minInputs = sopInfo.min_inputs;
  info->customOPInfo.maxInputs = sopInfo.max_inputs;
}

DLLEXPORT
SOP_CPlusPlusBase *CreateSOPInstance(const OP_NodeInfo *info) {
  return new td_rs_sop::RustSOP(info);
}

DLLEXPORT
void DestroySOPInstance(SOP_CPlusPlusBase *instance) {
  delete (RustSOP *)instance;
}

};

namespace td_rs_sop {

    RustSOP::RustSOP(const OP_NodeInfo *) {
        sop = new BoxDynSop(sop_new());
    };

    RustSOP::~RustSOP() {
        dyn_sop_drop_in_place(sop);
    };

    void RustSOP::getGeneralInfo(SOP_GeneralInfo *ginfo, const OP_Inputs *inputs, void *) {
        auto info = sop->getGeneralInfo();
        ginfo->cookEveryFrameIfAsked = info.cook_every_frame_if_asked;
        ginfo->cookEveryFrame = info.cook_every_frame;
        ginfo->directToGPU = info.direct_to_gpu;
    }

    void RustSOP::execute(SOP_Output *output, const OP_Inputs *inputs, void *) {
        auto out = new SopOutput(output);
        auto in = new OperatorInput(inputs);
        auto sopIn = new SopOperatorInput(inputs);
        sop->execute(out, in, sopIn);
    }

    void RustSOP::executeVBO(SOP_VBOOutput *output, const OP_Inputs *inputs, void *) {
        auto out = new SopOutput(output);
        auto in = new OperatorInput(inputs);
        auto sopIn = new SopOperatorInput(inputs);
        sop->executeVBO(out, in, sopIn);
    }

    void RustSOP::setupParameters(OP_ParameterManager *manager, void *) {
        auto m = new ParameterManager(manager);
        sop->setupParams(m);
    }

    void RustSOP::getWarningString(OP_String *warning, void *) {
        warning->setString(sop->getWarningString().c_str());
    }

}