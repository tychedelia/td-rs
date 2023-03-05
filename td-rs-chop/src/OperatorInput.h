#include "CHOP_CPlusPlusBase.h"
#include <rust/cxx.h>

class ChopInput {
public:
    ChopInput(OP_Inputs in)
    virtual int32_t		getNumInputs() const = 0;

    virtual const OP_CHOPInput*		getInput(size_t index);
    virtual const OP_CHOPInput*		getPararameterInput(const rust::Str);
    virtual const OP_ObjectInput*	getPararameterObject(const rust::Str);
    virtual double		            getParamaterDouble(const char* name, int32_t index = 0);
    virtual rust::Slice<double>		getParamaterDouble2(const rust::Str);
    virtual rust::Slice<double>		getParamaterDouble3(const rust::Str);
    virtual rust::Slice<double>		getParameterDouble4(const rust::Str);
    virtual int32_t		            getParInt(const rust::Str name, int32_t index = 0);
    virtual rust::Slice<int32_t>	getParInt2(const rust::Str name);
    virtual rust::Slice<int32_t>	getParInt3(const rust::Str name);
    virtual rust::Slice<int32_t>	getParInt4(const rust::Str name);
    virtual rust::Str	            getParString(const rust::Str name);
    virtual rust::Str               getParFilePath(const rust::Str name);
    virtual rust::Slice<
                rust::Slice<double>
            >                       getRelativeTransform(const rust::Str from_name, const rust::Str to_name);
    virtual void		            enablePar(const rust::Str name, bool onoff);
//    virtual const OP_DATInput*		getDAT(const rust::Str path);
//    virtual const OP_TOPInput*		getTOP(const rust::Str path);
//    virtual const OP_CHOPInput*		getCHOP(const rust::Str path);
//    virtual const OP_ObjectInput*	getObject(const rust::Str path);
//    virtual const OP_SOPInput*		getParSOP(const rust::Str name);
//    virtual const OP_SOPInput*		getInputSOP(int32_t index);
//    virtual const OP_SOPInput*		getSOP(const rust::Str path);

    // only valid for C++ DAT operators
//    virtual const OP_DATInput*		getInputDAT(int32_t index) const = 0;

    // To use Python in your Plugin you need to fill the
    // customOPInfo.pythonVersion member in Fill*PluginInfo.
    //
    // The returned object, if not null should have its reference count decremented
    // or else a memorky leak will occur.
//    virtual PyObject*				getParPython(const char* name) const = 0;


    // Returns a class whose members gives you information about timing
    // such as FPS and delta-time since the last cook.
    // See OP_TimeInfo for more information
//    virtual const OP_TimeInfo*		getTimeInfo() const = 0;
};
