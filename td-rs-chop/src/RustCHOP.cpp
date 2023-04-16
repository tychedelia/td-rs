/* Shared Use License: This file is owned by Derivative Inc. (Derivative)
* and can only be used, and/or modified for use, in conjunction with
* Derivative's TouchDesigner software, and only if you are a licensee who has
* accepted Derivative's TouchDesigner license or assignment agreement
* (which also govern the use of this file). You may share or redistribute
* a modified version of this file provided the following conditions are met:
*
* 1. The shared file or redistribution must retain the information set out
* above and this list of conditions.
* 2. Derivative's name (Derivative Inc.) or its trademarks may not be used
* to endorse or promote products derived from this file without specific
* prior written permission from Derivative.
*/

#include "RustCHOP.h"
#include "BoxDynChop.h"
#include "ChopOutput.h"
#include "ParameterManager.h"
#include <td-rs-chop/src/cxx.rs.h>
#include <rust/cxx.h>
#include <stdio.h>
#include <string.h>
#include <cmath>
#include <assert.h>
#include <functional>

using namespace td_rs_base::ffi;

// These functions are basic C function, which the DLL loader can find
// much easier than finding a C++ Class.
// The DLLEXPORT prefix is needed so the compile exports these functions from the .dll
// you are creating
extern "C"
{

DLLEXPORT
void
FillCHOPPluginInfo(CHOP_PluginInfo *info) {
    OperatorInfo chopInfo = chop_get_operator_info();
    info->apiVersion = CHOPCPlusPlusAPIVersion;
    info->customOPInfo.opType->setString(chopInfo.operator_type.c_str());
    info->customOPInfo.opLabel->setString(chopInfo.operator_label.c_str());
    info->customOPInfo.authorName->setString(chopInfo.author_name.c_str());
    info->customOPInfo.authorEmail->setString(chopInfo.author_email.c_str());
    info->customOPInfo.minInputs = chopInfo.min_inputs;
    info->customOPInfo.maxInputs = chopInfo.max_inputs;
}

DLLEXPORT
CHOP_CPlusPlusBase *
CreateCHOPInstance(const OP_NodeInfo *info) {
    return new RustCHOP(info);
}

DLLEXPORT
void
DestroyCHOPInstance(CHOP_CPlusPlusBase *instance) {
    delete (RustCHOP *) instance;
}

};


RustCHOP::RustCHOP(const OP_NodeInfo *info) {
    chop = new BoxDynChop(chop_new());
}

RustCHOP::~RustCHOP() {
    dyn_chop_drop_in_place(chop);
}

void
RustCHOP::getGeneralInfo(CHOP_GeneralInfo *ginfo, const OP_Inputs *inputs, void *reserved1) {
    auto info = chop->getGeneralInfo();
    ginfo->cookEveryFrameIfAsked = info.cook_every_frame_if_asked;
    ginfo->cookEveryFrame = info.cook_every_frame;
    ginfo->timeslice = info.timeslice;
    ginfo->inputMatchIndex = info.input_match_index;
}

bool
RustCHOP::getOutputInfo(CHOP_OutputInfo *info, const OP_Inputs *inputs, void *reserved1) {
    ChopOutputInfo ci;
    auto in = new OperatorInput(inputs);
    auto is_output = chop->getOutputInfo(&ci, in);

    info->numChannels = ci.num_channels;
    info->sampleRate = ci.sample_rate;
    info->numSamples = ci.num_samples;
    info->startIndex = ci.start_index;

    return is_output;
}

void
RustCHOP::getChannelName(int32_t index, OP_String *name, const OP_Inputs *inputs, void *reserved1) {
    auto in = new OperatorInput(inputs);
    name->setString(chop->getChannelName(index, in).c_str());
}

void
RustCHOP::execute(CHOP_Output *output,
                  const OP_Inputs *inputs,
                  void *reserved) {
    auto out = new ChopOutput(output);
    auto in = new OperatorInput(inputs);
    chop->execute(out, in);
}

int32_t
RustCHOP::getNumInfoCHOPChans(void *reserved1) {
    return chop->getNumInfoChopChans();
}

void
RustCHOP::getInfoCHOPChan(int32_t index,
                          OP_InfoCHOPChan *chan,
                          void *reserved1) {
    auto c = chop->getInfoChopChan(index);
    chan->name->setString(c.name.c_str());
    c.value = c.value;
}

bool
RustCHOP::getInfoDATSize(OP_InfoDATSize *infoSize, void *reserved1) {
    ChopInfoDatSize datSize;
    return chop->getInfoDatSize(&datSize);
}

void
RustCHOP::getInfoDATEntries(int32_t index,
                            int32_t nEntries,
                            OP_InfoDATEntries *entries,
                            void *reserved1) {

    ChopInfoDatEntries ents;
    chop->getInfoDATEntries(index, nEntries, &ents);

    assert(nEntries == ents.values.size());

    for (auto i = 0; i < nEntries; i++) {
        auto e = ents.values[i];
        entries->values[i]->setString(e.c_str());
    }
}

void
RustCHOP::setupParameters(OP_ParameterManager *manager, void *reserved1) {
    auto m = new ParameterManager(manager);
    chop->setupParams(m);
}

void
RustCHOP::getWarningString(OP_String *warning, void *reserved1) {
    warning->setString(chop->getWarningString().c_str());
}

void
RustCHOP::getErrorString(OP_String *error, void *reserved1) {
    error->setString(chop->getErrorString().c_str());
}

void
RustCHOP::getInfoPopupString(OP_String *info, void *reserved1) {
    info->setString(chop->getInfoString().c_str());
}

void
RustCHOP::pulsePressed(const char *name, void *reserved1) {
    if (!strcmp(name, "Reset")) {
        chop->onReset();
    }
}
