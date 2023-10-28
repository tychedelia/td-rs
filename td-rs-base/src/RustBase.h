#include "CPlusPlus_Common.h"
#include <memory>
#include "Python.h"
#include <vector>
#include <iostream>

#ifndef TD_RS_RUSTBASE_H
#define TD_RS_RUSTBASE_H

void setString(TD::OP_String *dest, const char *src) { dest->setString(src); }

std::unique_ptr<TD::PY_Context> getPyContext(TD::PY_Struct *pyStruct) {
    auto ctx = pyStruct->context;
    return std::unique_ptr<TD::PY_Context>(ctx);
}

std::unique_ptr<TD::OP_Context> getOpContext(TD::OP_Context *ctx) {
    return std::unique_ptr<TD::OP_Context>(ctx);
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

#endif // TD_RS_RUSTBASE_H
