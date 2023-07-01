use ref_cast::RefCast;
use crate::cxx::OP_TOPInput;

#[repr(transparent)]
#[derive(RefCast)]
pub struct TopInput {
    input: OP_TOPInput,
}
