#include "CPlusPlus_Common.h"
#include "RustSopPlugin.h"
#include <iostream>

extern "C" {

RustSopPlugin *sop_new();
void sop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillSOPPluginInfo(SOP_PluginInfo *info) {
    info->apiVersion = SOPCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    sop_get_plugin_info_impl(*opInfo);
}

DLLEXPORT
SOP_CPlusPlusBase *CreateSOPInstance(const OP_NodeInfo *info) {
    return sop_new();
}

DLLEXPORT
void DestroySOPInstance(SOP_CPlusPlusBase *instance) {
    delete (RustSopPlugin *) instance;
}

}