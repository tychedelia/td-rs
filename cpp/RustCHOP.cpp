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
#include "lib.rs.h"
#include "cxx.h"
#include <stdio.h>
#include <string.h>
#include <cmath>
#include <assert.h>
#include <functional>
#include <iostream>

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
    ChopOperatorInputs opIn;
    for (auto i = 0; i < inputs->getNumInputs(); i++) {
        auto ci = inputs->getInputCHOP(i);
        auto in = mapInput(ci);
        opIn.inputs.push_back(in);
    }

    auto is_output = chop->getOutputInfo(&ci, &opIn);

    info->numChannels = ci.num_channels;
    info->sampleRate = ci.sample_rate;
    info->numSamples = ci.num_samples;
    info->startIndex = ci.start_index;

    return is_output;
}

void
RustCHOP::getChannelName(int32_t index, OP_String *name, const OP_Inputs *inputs, void *reserved1) {
    ChopOperatorInputs opIn;
    for (auto i = 0; i < inputs->getNumInputs(); i++) {
        auto ci = inputs->getInputCHOP(i);
        auto in = mapInput(ci);
        opIn.inputs.push_back(in);
    }
    name->setString(chop->getChannelName(index, &opIn).c_str());
}

void
RustCHOP::execute(CHOP_Output *output,
                  const OP_Inputs *inputs,
                  void *reserved) {
    ChopOperatorInputs ins;
    ins.inputs = rust::Vec<ChopOperatorInput>();
    for (int i = 0; i < inputs->getNumInputs(); i++) {
        auto cinput = inputs->getInputCHOP(i);
        auto in = mapInput(cinput);
        ins.inputs.push_back(in);
    }

    ChopOutput out;
    out.channels = rust::Vec<ChopChannel>();
    out.num_channels = output->numChannels;
    out.num_samples = output->numSamples;
    out.sample_rate = output->sampleRate;
    for (auto i = 0; i < out.num_channels; i++) {
        ChopChannel c;
        out.channels.push_back(c);
    }
    chop->execute(&out, &ins);
    for (auto i = 0; i < out.channels.size(); i++) {
        auto c = out.channels[i];
        std::copy(std::begin(c.data), std::end(c.data), output->channels[i]);
    }
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
    for (auto param: chop->getParams().numeric_params) {
        OP_NumericParameter np;

        np.name = param.name.c_str();
        np.label = param.label.c_str();
        np.page = param.page.c_str();
        std::copy(std::begin(param.default_values), std::end(param.default_values), std::begin(np.defaultValues));
        std::copy(std::begin(param.max_values), std::end(param.max_values), std::begin(np.maxValues));
        std::copy(std::begin(param.min_values), std::end(param.min_values), std::begin(np.minValues));
        std::copy(std::begin(param.max_sliders), std::end(param.max_sliders), std::begin(np.maxSliders));
        std::copy(std::begin(param.min_sliders), std::end(param.min_sliders), std::begin(np.minSliders));
        std::copy(std::begin(param.clamp_maxes), std::end(param.clamp_maxes), std::begin(np.clampMaxes));
        std::copy(std::begin(param.clamp_mins), std::end(param.clamp_mins), std::begin(np.clampMaxes));

        OP_ParAppendResult res = manager->appendFloat(np);
        assert(res == OP_ParAppendResult::Success);
    }

    for (auto param: chop->getParams().string_params) {
        OP_StringParameter sp;

        sp.name = param.name.c_str();
        sp.label = param.label.c_str();
        sp.page = param.page.c_str();
        sp.defaultValue = param.default_value.c_str();

        OP_ParAppendResult res = manager->appendString(sp);
        assert(res == OP_ParAppendResult::Success);
    }

    for (auto param : chop->getParams().pulse_params) {
        OP_NumericParameter np;

        np.name = param.name.c_str();
        np.label = param.label.c_str();
        np.page = param.page.c_str();

        OP_ParAppendResult res = manager->appendPulse(np);
        assert(res == OP_ParAppendResult::Success);
    }
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

ChopOperatorInput
RustCHOP::mapInput(const OP_CHOPInput *input) {
    ChopOperatorInput chop;
    chop.id = input->opId;
    chop.path = input->opPath;
    chop.num_channels = input->numChannels;
    chop.num_samples = input->numSamples;
    chop.sample_rate = input->sampleRate;
    chop.start_index = input->startIndex;
    chop.channels = rust::Vec<ChopChannel>();
    int ind = 0;
    for (auto i = 0; i < input->numChannels; i++) {
        ChopChannel chan;
        chan.data = rust::Vec<float>();
        for (auto j = 0; j < input->numSamples; j++) {
            chan.data.push_back(input->getChannelData(i)[ind]);
            ind++;
            ind = ind % input->numSamples;
        }
        chop.channels.push_back(chan);
    }
    return chop;
}

