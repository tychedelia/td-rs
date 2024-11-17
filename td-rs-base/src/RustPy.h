#include "CPlusPlus_Common.h"
#include <memory>
#ifdef PYTHON_ENABLED
#include <Python.h>
#endif
#include <vector>
#include <iostream>

#ifndef TD_RS_RUSTPY_H
#define TD_RS_RUSTPY_H

#ifdef PYTHON_ENABLED
TD::PY_Context* getPyContext(TD::PY_Struct *pyStruct) {
    return pyStruct->context;
}

void setPyInfo(TD::OP_CustomOPInfo &opInfo, void *pymethods, size_t size, void *pygetsets, size_t getsetsize) {
    if (size == 0) {
        std::cout << "No methods" << std::endl;
        opInfo.pythonMethods = nullptr;
    } else {
        opInfo.pythonMethods = static_cast<PyMethodDef*>(pymethods);
    }

    if (getsetsize == 0) {
        std::cout << "No getsets" << std::endl;
        opInfo.pythonGetSets = nullptr;
    } else {
        opInfo.pythonGetSets = static_cast<PyGetSetDef*>(pygetsets);
    }
}
#else

std::unique_ptr<TD::PY_Context> getPyContext(TD::PY_Struct *pyStruct) {
    return nullptr;
}

void setPyInfo(TD::OP_CustomOPInfo &opInfo, void *pymethods, size_t size, void *pygetsets, size_t getsetsize) {
    std::cout << "Python is not enabled" << std::endl;
}

#endif

#endif //TD_RS_RUSTPY_H
