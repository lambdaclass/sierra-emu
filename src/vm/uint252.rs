use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        int::unsigned256::Uint256Concrete,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;
use starknet_crypto::Felt;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Uint256Concrete,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Uint256Concrete::IsZero(_) => todo!(),
        Uint256Concrete::Divmod(_) => todo!(),
        Uint256Concrete::SquareRoot(_) => todo!(),
        Uint256Concrete::InvModN(_) => todo!(),
    }
}
