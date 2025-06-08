#include "CPlusPlus_Common.h"
#include "RustSopPlugin.h"
#include <iostream>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif

extern "C" {

RustSopPlugin *sop_new(const OP_NodeInfo &info);
void sop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillSOPPluginInfo(SOP_PluginInfo *info) {
    info->apiVersion = SOPCPlusPlusAPIVersion;
    auto opInfo = &info->customOPInfo;
    sop_get_plugin_info_impl(*opInfo);
#ifdef PYTHON_ENABLED
    opInfo->pythonVersion->setString(PY_VERSION);
#endif
}

DLLEXPORT
SOP_CPlusPlusBase *CreateSOPInstance(const OP_NodeInfo *info) {
    return sop_new(*info);
}

DLLEXPORT
void DestroySOPInstance(SOP_CPlusPlusBase *instance) {
    delete (RustSopPlugin *) instance;
}

}