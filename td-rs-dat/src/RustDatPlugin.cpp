#include "CPlusPlus_Common.h"
#include "RustDatPlugin.h"
#include <iostream>

extern "C" {

RustDatPlugin *dat_new(const OP_NodeInfo &info);
void dat_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillDATPluginInfo(DAT_PluginInfo *info) {
    info->apiVersion = DATCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    dat_get_plugin_info_impl(*opInfo);
}

DLLEXPORT
DAT_CPlusPlusBase *CreateDATInstance(const OP_NodeInfo *info) {
    return dat_new(*info);
}

DLLEXPORT
void DestroyDATInstance(DAT_CPlusPlusBase *instance) {
    delete (RustDatPlugin *) instance;
}

}