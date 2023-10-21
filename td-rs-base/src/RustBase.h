#include "CPlusPlus_Common.h"
#include <memory>

#ifndef TD_RS_RUSTBASE_H
#define TD_RS_RUSTBASE_H

void setString(TD::OP_String *dest, const char *src) { dest->setString(src); }

std::unique_ptr<TD::PY_Context> getPyContext(TD::PY_Struct *pyStruct) {
    auto ctx = pyStruct->context;
    return std::unique_ptr<TD::PY_Context>(ctx);
}

#endif // TD_RS_RUSTBASE_H
