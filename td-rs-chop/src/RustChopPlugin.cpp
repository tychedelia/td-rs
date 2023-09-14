#include "RustChopPlugin.h"
#include "CPlusPlus_Common.h"
#include <iostream>

extern "C" {

RustChopPlugin *chop_new();
void chop_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillCHOPPluginInfo(CHOP_PluginInfo *info) {
  info->apiVersion = CHOPCPlusPlusAPIVersion;
  auto opInfo = &info->customOPInfo;
  chop_get_plugin_info_impl(*opInfo);
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
