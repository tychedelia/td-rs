#include "SOP_CPlusPlusBase.h"
#include <string>

namespace td_rs_sop {

    class RustSOP : public SOP_CPlusPlusBase {
    public:
        RustSOP(const OP_NodeInfo *info);

        virtual ~RustSOP();

        virtual void getGeneralInfo(SOP_GeneralInfo *, const OP_Inputs *,
                                    void *) override;

        virtual void execute(SOP_Output *, const OP_Inputs *, void *) override;

        virtual void executeVBO(SOP_VBOOutput *, const OP_Inputs *, void *) override;

        virtual void setupParameters(OP_ParameterManager *manager, void *) override;

        virtual void getWarningString(OP_String *, void *) override;

    private:
        BoxDynSop *sop;
    };

}