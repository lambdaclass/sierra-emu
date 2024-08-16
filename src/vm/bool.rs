use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        boolean::BoolConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
        lib_func::{SignatureAndTypeConcreteLibfunc, SignatureOnlyConcreteLibfunc},
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &BoolConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        BoolConcreteLibfunc::And(info) => eval_and(registry, info, args),
        BoolConcreteLibfunc::Not(_) => todo!(),
        BoolConcreteLibfunc::Xor(_) => todo!(),
        BoolConcreteLibfunc::Or(_) => todo!(),
        BoolConcreteLibfunc::ToFelt252(_) => todo!(),
    }
}

pub fn eval_and(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    dbg!(&args);
    let [Value::Enum {
        self_ty,
        index: lhs,
        payload,
    }, Value::Enum {
        self_ty: _,
        index: rhs,
        payload: _,
    }]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    EvalAction::NormalBranch(
        0,
        smallvec![Value::Enum {
            self_ty,
            index: lhs & rhs,
            payload
        }],
    )
}
