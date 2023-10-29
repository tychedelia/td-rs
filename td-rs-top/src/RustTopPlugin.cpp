#include "RustTopPlugin.h"
#include "CPlusPlus_Common.h"
#include <iostream>

extern "C" {

RustTopPlugin *top_new(const OP_NodeInfo &info, TOP_Context &context);
TOP_ExecuteMode top_get_plugin_info_impl(OP_CustomOPInfo &opInfo);

DLLEXPORT
void FillTOPPluginInfo(TOP_PluginInfo *info) {
  info->apiVersion = TOPCPlusPlusAPIVersion;
  auto opInfo = &info->customOPInfo;
  auto mode = top_get_plugin_info_impl(*opInfo);
  info->executeMode = mode;
}

DLLEXPORT
TOP_CPlusPlusBase *CreateTOPInstance(const OP_NodeInfo *info, TOP_Context *context) {
  return top_new(*info, *context);
}

DLLEXPORT
void DestroyTOPInstance(TOP_CPlusPlusBase *instance) {
  delete (RustTopPlugin *)instance;
}
}