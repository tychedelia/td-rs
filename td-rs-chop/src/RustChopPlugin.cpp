#include "RustChopPlugin.h"
#include "CPlusPlus_Common.h"
#include <iostream>
#include <Python.h>

extern "C" {

RustChopPlugin *chop_new();
void chop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillCHOPPluginInfo(CHOP_PluginInfo *info) {
  info->apiVersion = CHOPCPlusPlusAPIVersion;
  auto opInfo = &info->customOPInfo;
  chop_get_plugin_info_impl(*opInfo);
  opInfo->pythonVersion->setString(PY_VERSION);
}

DLLEXPORT
CHOP_CPlusPlusBase *CreateCHOPInstance(const OP_NodeInfo *info) {
  return chop_new();
}

DLLEXPORT
void DestroyCHOPInstance(CHOP_CPlusPlusBase *instance) {
  delete (RustChopPlugin *)instance;
}
}
