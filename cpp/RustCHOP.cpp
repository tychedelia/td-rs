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
#include "lib.rs.h"
#include "cxx.h"
#include <stdio.h>
#include <string.h>
#include <cmath>
#include <assert.h>
#include <functional>

// These functions are basic C function, which the DLL loader can find
// much easier than finding a C++ Class.
// The DLLEXPORT prefix is needed so the compile exports these functions from the .dll
// you are creating
extern "C"
{

DLLEXPORT
void
FillCHOPPluginInfo(CHOP_PluginInfo *info)
{
    Chop chop = get_chop();
	info->apiVersion = CHOPCPlusPlusAPIVersion;
	info->customOPInfo.opType->setString(chop.info.operator_type.c_str());
	info->customOPInfo.opLabel->setString(chop.info.operator_label.c_str());
	info->customOPInfo.authorName->setString(chop.info.author_name.c_str());
	info->customOPInfo.authorEmail->setString(chop.info.author_email.c_str());
	info->customOPInfo.minInputs = chop.info.min_inputs;
	info->customOPInfo.maxInputs = chop.info.max_inputs;
}

DLLEXPORT
CHOP_CPlusPlusBase*
CreateCHOPInstance(const OP_NodeInfo* info)
{
	// Return a new instance of your class every time this is called.
	// It will be called once per CHOP that is using the .dll
	return new RustCHOP(info);
}

DLLEXPORT
void
DestroyCHOPInstance(CHOP_CPlusPlusBase* instance)
{
	// Delete the instance here, this will be called when
	// Touch is shutting down, when the CHOP using that instance is deleted, or
	// if the CHOP loads a different DLL
	delete (RustCHOP*)instance;
}

};


RustCHOP::RustCHOP(const OP_NodeInfo* info)
{
    chop = get_chop();
}

RustCHOP::~RustCHOP()
{

}

void
RustCHOP::getGeneralInfo(CHOP_GeneralInfo* ginfo, const OP_Inputs* inputs, void* reserved1)
{
	// This will cause the node to cook every frame
	ginfo->cookEveryFrameIfAsked = true;

	// Note: To disable timeslicing you'll need to turn this off, as well as ensure that
	// getOutputInfo() returns true, and likely also set the info->numSamples to how many
	// samples you want to generate for this CHOP. Otherwise it'll take on length of the
	// input CHOP, which may be timesliced.
	ginfo->timeslice = true;

	ginfo->inputMatchIndex = 0;
}

bool
RustCHOP::getOutputInfo(CHOP_OutputInfo* info, const OP_Inputs* inputs, void* reserved1)
{
	// If there is an input connected, we are going to match it's channel names etc
	// otherwise we'll specify our own.
	if (inputs->getNumInputs() > 0)
	{
		return false;
	}
	else
	{
		info->numChannels = 1;

		// Since we are outputting a timeslice, the system will dictate
		// the numSamples and start_index of the CHOP data
		//info->numSamples = 1;
		//info->start_index = 0

		// For illustration we are going to output 120hz data
		info->sampleRate = 120;
		return true;
	}
}

void
RustCHOP::getChannelName(int32_t index, OP_String *name, const OP_Inputs* inputs, void* reserved1)
{
	name->setString("chan1");
}

void
RustCHOP::execute(CHOP_Output* output,
							  const OP_Inputs* inputs,
							  void* reserved)
{
    for (int i = 0; i < inputs->getNumInputs(); i++) {
        auto cinput = inputs->getInputCHOP(i);
        ChopOperatorInputs chop;
        chop.id = cinput->opId;
        chop.path = cinput->opPath;
        chop.num_channels = cinput->numChannels;
        chop.num_samples = cinput->numSamples;
        chop.sample_rate = cinput->sampleRate;
        chop.start_index = cinput->startIndex;
        chop.inputs = rust::Vec<ChopInput>();
        int ind = 0;
        for (auto i = 0; i < cinput->numChannels; i++) {
            ChopInput input;
            input.data = rust::Vec<float>();
            for (auto j = 0; j < cinput->numSamples; j++) {
                input.data.push_back(cinput->getChannelData(i)[ind]);
                ind++;
                ind = ind % cinput->numSamples;
            }
            chop.inputs.push_back(input);
        }
        chop_execute(chop);
    }
}

int32_t
RustCHOP::getNumInfoCHOPChans(void * reserved1)
{
	// We return the number of channel we want to output to any Info CHOP
	// connected to the CHOP. In this example we are just going to send one channel.
	return 2;
}

void
RustCHOP::getInfoCHOPChan(int32_t index,
										OP_InfoCHOPChan* chan,
										void* reserved1)
{

    // TODO:
}

bool		
RustCHOP::getInfoDATSize(OP_InfoDATSize* infoSize, void* reserved1)
{
	infoSize->rows = 2;
	infoSize->cols = 2;
	// Setting this to false means we'll be assigning values to the table
	// one row at a time. True means we'll do it one column at a time.
	infoSize->byColumn = false;
	return true;
}

void
RustCHOP::getInfoDATEntries(int32_t index,
										int32_t nEntries,
										OP_InfoDATEntries* entries, 
										void* reserved1)
{
    // TODO
}

void
RustCHOP::setupParameters(OP_ParameterManager* manager, void *reserved1)
{

}

void 
RustCHOP::pulsePressed(const char* name, void* reserved1)
{
	if (!strcmp(name, "Reset"))
	{
	    // TODO
    }
}

