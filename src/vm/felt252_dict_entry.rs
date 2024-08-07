use super::EvalAction;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        felt252_dict::Felt252DictEntryConcreteLibfunc,
        lib_func::SignatureAndTypeConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Felt252DictEntryConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        Felt252DictEntryConcreteLibfunc::Get(info) => eval_get(registry, info, args),
        Felt252DictEntryConcreteLibfunc::Finalize(_) => todo!(),
    }
}

pub fn eval_get(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    dbg!(_info.signature.param_signatures.len());
    dbg!(&_info.signature.param_signatures[0].ty);
    dbg!(&_info.signature.param_signatures[1].ty);
    dbg!(_info.signature.branch_signatures.len());

    let [Value::FeltDict { ty, data }, Value::Felt(key)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    EvalAction::NormalBranch(0, smallvec![Value::FeltDictEntry { ty, data, key }])
}
