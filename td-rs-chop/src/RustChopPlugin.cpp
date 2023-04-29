#include "CPlusPlus_Common.h"
#include "RustChopPlugin.h"

std::unique_ptr<RustChopPlugin> chop_new();

extern "C" {

DLLEXPORT
void FillCHOPPluginInfo(CHOP_PluginInfo *info) {
}

DLLEXPORT
CHOP_CPlusPlusBase* CreateCHOPInstance(const OP_NodeInfo *info) {
    return chop_new().release();
}

DLLEXPORT
void DestroyCHOPInstance(CHOP_CPlusPlusBase *instance) {
    delete (RustChopPlugin*) instance;
}

}