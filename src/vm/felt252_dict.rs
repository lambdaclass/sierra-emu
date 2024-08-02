use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        felt252_dict::Felt252DictConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use sierra_emu::Value;
use std::collections::HashMap;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &Felt252DictConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    match selector {
        Felt252DictConcreteLibfunc::New(info) => eval_new(registry, info, args),
        Felt252DictConcreteLibfunc::Squash(_) => todo!(),
    }
}

pub fn eval_new(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], Value::Unit); // SegmentArena

    let type_info = registry
        .get_type(&info.signature.branch_signatures[0].vars[1].ty)
        .unwrap();
    let ty = match type_info {
        CoreTypeConcrete::Felt252Dict(info) => info.ty.clone(),
        _ => unreachable!(),
    };

    (
        Some(0),
        vec![
            args[0].clone(),
            Value::FeltDict {
                ty,
                data: HashMap::new(),
            },
        ],
    )
}
