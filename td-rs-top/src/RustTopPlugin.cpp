#include "CPlusPlus_Common.h"
#include "RustTopPlugin.h"
#include <iostream>

extern "C" {

RustTopPlugin *top_new();
void top_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillTOPPluginInfo(TOP_PluginInfo *info) {
    info->apiVersion = TOPCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    top_get_plugin_info_impl(*opInfo);
}

DLLEXPORT
        TOP_CPlusPlusBase *CreateTOPInstance(const OP_NodeInfo *info) {
    return top_new();
}

DLLEXPORT
void DestroyTOPInstance(TOP_CPlusPlusBase *instance) {
    delete (RustTopPlugin *) instance;
}

}