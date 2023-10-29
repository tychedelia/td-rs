#include "RustChopPlugin.h"
#include "CPlusPlus_Common.h"
#include <iostream>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif

extern "C" {

RustChopPlugin *chop_new(const OP_NodeInfo &info);
void chop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillCHOPPluginInfo(CHOP_PluginInfo *info) {
  info->apiVersion = CHOPCPlusPlusAPIVersion;
  auto opInfo = &info->customOPInfo;
  chop_get_plugin_info_impl(*opInfo);
#ifdef PYTHON_ENABLED
  opInfo->pythonVersion->setString(PY_VERSION);
#endif
}

DLLEXPORT
CHOP_CPlusPlusBase *CreateCHOPInstance(const OP_NodeInfo *info) {
  return chop_new(*info);
}

DLLEXPORT
void DestroyCHOPInstance(CHOP_CPlusPlusBase *instance) {
  delete (RustChopPlugin *)instance;
}
}
