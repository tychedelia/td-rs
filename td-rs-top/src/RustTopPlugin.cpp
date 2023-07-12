#include "CPlusPlus_Common.h"
#include "RustTopPlugin.h"
#include <iostream>

extern "C" {

RustTopPlugin *top_new();
TOP_ExecuteMode top_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillTOPPluginInfo(TOP_PluginInfo *info) {
    info->apiVersion = TOPCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    auto mode = top_get_plugin_info_impl(*opInfo);
    info->executeMode = mode;
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