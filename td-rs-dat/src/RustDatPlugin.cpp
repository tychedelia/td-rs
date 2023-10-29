#include "CPlusPlus_Common.h"
#include "RustDatPlugin.h"
#include <iostream>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif


extern "C" {

RustDatPlugin *dat_new(const OP_NodeInfo &info);
void dat_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillDATPluginInfo(DAT_PluginInfo *info) {
    info->apiVersion = DATCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    dat_get_plugin_info_impl(*opInfo);
#ifdef PYTHON_ENABLED
    opInfo->pythonVersion->setString(PY_VERSION);
#endif
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