use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        felt252_dict::Felt252DictConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &'a Felt252DictConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    match selector {
        Felt252DictConcreteLibfunc::New(info) => eval_new(registry, info, args),
        Felt252DictConcreteLibfunc::Squash(_) => todo!(),
    }
}

pub fn eval_new<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    info: &'a SignatureOnlyConcreteLibfunc,
    args: Vec<Value<'a>>,
) -> EvalAction<'a> {
    let [segment_arena @ Value::Unit]: [Value<'a>; 1] = args.try_into().unwrap() else {
        panic!()
    };

    let type_info = registry
        .get_type(&info.signature.branch_signatures[0].vars[1].ty)
        .unwrap();
    let ty = match type_info {
        CoreTypeConcrete::Felt252Dict(info) => &info.ty,
        _ => unreachable!(),
    };

    EvalAction::NormalBranch(
        0,
        smallvec![
            segment_arena,
            Value::FeltDict {
                ty,
                data: Rc::new(RefCell::new(HashMap::new())),
            },
        ],
    )
}
