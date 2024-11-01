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
        return;
    }
    PyMethodDef *pm = (PyMethodDef*) pymethods;
    std::vector<PyMethodDef> *methods = new std::vector<PyMethodDef>();
    for (size_t i = 0; i < size; i++) {
        methods->push_back(pm[i]);
    }
    PyMethodDef *arr = methods->data();
    opInfo.pythonMethods = arr;

    if (getsetsize == 0) {
        return;
    }
    PyGetSetDef *pgs = (PyGetSetDef*) pygetsets;
    std::vector<PyGetSetDef> *getsets = new std::vector<PyGetSetDef>();
    for (size_t i = 0; i < getsetsize; i++) {
        getsets->push_back(pgs[i]);
    }
    PyGetSetDef *arr2 = getsets->data();
    opInfo.pythonGetSets = arr2;
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
