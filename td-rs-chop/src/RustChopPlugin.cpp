#include "CPlusPlus_Common.h"
#include "RustChopPlugin.h"
#include <iostream>


extern "C" {

RustChopPluginWrapper* chop_new();

DLLEXPORT
void FillCHOPPluginInfo(CHOP_PluginInfo *info) {
    info->apiVersion = CHOPCPlusPlusAPIVersion;
}

DLLEXPORT
CHOP_CPlusPlusBase* CreateCHOPInstance(const OP_NodeInfo *info) {
    std::cout << "CreateCHOPInstance" << std::endl;
    auto chop = chop_new();
    std::cout << chop->getNumInfoCHOPChans() << std::endl;
//    std::cout << chop->getNumInfoCHOPChans(0) << std::endl;

//    chop->pulsePressed(nullptr, 0);
//    chop->pulsePressed(nullptr, 0);
//    chop->pulsePressed(nullptr, 0);

    // this blows up
//    chop->setupParameters(nullptr, 0);
    return chop;
}

DLLEXPORT
void DestroyCHOPInstance(CHOP_CPlusPlusBase *instance) {
    delete (RustChopPlugin*) instance;
}

}