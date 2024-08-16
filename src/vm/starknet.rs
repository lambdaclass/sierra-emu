use super::EvalAction;
use crate::{starknet::StarknetSyscallHandler, Value};
use cairo_lang_sierra::{
    extensions::{
        consts::SignatureAndConstConcreteLibfunc,
        core::{CoreLibfunc, CoreType, CoreTypeConcrete},
        lib_func::SignatureOnlyConcreteLibfunc,
        starknet::StarkNetConcreteLibfunc,
        ConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;
use starknet_types_core::felt::Felt;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &StarkNetConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    match selector {
        StarkNetConcreteLibfunc::CallContract(info) => {
            self::eval_call_contract(registry, info, args)
        }
        StarkNetConcreteLibfunc::ClassHashConst(info) => {
            eval_class_hash_const(registry, info, args)
        }
        StarkNetConcreteLibfunc::ClassHashTryFromFelt252(info) => {
            eval_class_hash_try_from_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::ClassHashToFelt252(info) => {
            eval_class_hash_to_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::ContractAddressConst(info) => {
            eval_contract_address_const(registry, info, args)
        }
        StarkNetConcreteLibfunc::ContractAddressTryFromFelt252(info) => {
            eval_contract_address_try_from_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::ContractAddressToFelt252(info) => {
            eval_contract_address_to_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageRead(info) => {
            eval_storage_read(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::StorageWrite(info) => {
            eval_storage_write(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::StorageBaseAddressConst(info) => {
            eval_storage_base_address_const(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageBaseAddressFromFelt252(info) => {
            eval_storage_base_address_from_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageAddressFromBase(info) => {
            eval_storage_address_from_base(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageAddressFromBaseAndOffset(info) => {
            eval_storage_address_from_base_and_offset(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageAddressToFelt252(info) => {
            eval_storage_address_to_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageAddressTryFromFelt252(info) => {
            eval_storage_address_try_from_felt(registry, info, args)
        }
        StarkNetConcreteLibfunc::EmitEvent(info) => {
            eval_emit_event(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::GetBlockHash(info) => {
            eval_get_block_hash(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::GetExecutionInfo(info) => {
            eval_get_execution_info(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::GetExecutionInfoV2(info) => {
            eval_get_execution_info_v2(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::Deploy(info) => eval_deploy(registry, info, args),
        StarkNetConcreteLibfunc::Keccak(info) => eval_keccak(registry, info, args),
        StarkNetConcreteLibfunc::Sha256ProcessBlock(_) => todo!(),
        StarkNetConcreteLibfunc::Sha256StateHandleInit(_) => todo!(),
        StarkNetConcreteLibfunc::Sha256StateHandleDigest(_) => todo!(),
        StarkNetConcreteLibfunc::LibraryCall(info) => eval_library_call(registry, info, args),
        StarkNetConcreteLibfunc::ReplaceClass(info) => eval_replace_class(registry, info, args),
        StarkNetConcreteLibfunc::SendMessageToL1(info) => {
            eval_send_message_to_l1(registry, info, args)
        }
        StarkNetConcreteLibfunc::Testing(info) => todo!(),
        StarkNetConcreteLibfunc::Secp256(info) => todo!(),
    }
}

fn eval_class_hash_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndConstConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::Felt(info.c.clone().into())])
}

fn eval_storage_base_address_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndConstConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::Felt(info.c.clone().into())])
}

fn eval_contract_address_const(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureAndConstConcreteLibfunc,
    _args: Vec<Value>,
) -> EvalAction {
    EvalAction::NormalBranch(0, smallvec![Value::Felt(info.c.clone().into())])
}

fn eval_class_hash_try_from_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    // 2 ** 251 = 3618502788666131106986593281521497120414687020801267626233049500247285301248

    let [range_check @ Value::Unit, Value::Felt(value)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    if value
        < Felt::from_dec_str(
            "3618502788666131106986593281521497120414687020801267626233049500247285301248",
        )
        .unwrap()
    {
        EvalAction::NormalBranch(0, smallvec![range_check, Value::Felt(value)])
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check])
    }
}

fn eval_contract_address_try_from_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    // 2 ** 251 = 3618502788666131106986593281521497120414687020801267626233049500247285301248

    let [range_check @ Value::Unit, Value::Felt(value)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    if value
        < Felt::from_dec_str(
            "3618502788666131106986593281521497120414687020801267626233049500247285301248",
        )
        .unwrap()
    {
        EvalAction::NormalBranch(0, smallvec![range_check, Value::Felt(value)])
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check])
    }
}

fn eval_storage_address_try_from_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    // 2 ** 251 = 3618502788666131106986593281521497120414687020801267626233049500247285301248

    let [range_check @ Value::Unit, Value::Felt(value)]: [Value; 2] = args.try_into().unwrap()
    else {
        panic!()
    };

    if value
        < Felt::from_dec_str(
            "3618502788666131106986593281521497120414687020801267626233049500247285301248",
        )
        .unwrap()
    {
        EvalAction::NormalBranch(0, smallvec![range_check, Value::Felt(value)])
    } else {
        EvalAction::NormalBranch(1, smallvec![range_check])
    }
}

fn eval_storage_base_address_from_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check, value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![range_check, value])
}

fn eval_storage_address_to_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![value])
}

fn eval_contract_address_to_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![value])
}

fn eval_class_hash_to_felt(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![value])
}

fn eval_storage_address_from_base(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![value])
}

fn eval_storage_address_from_base_and_offset(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Felt(value), Value::U8(offset)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    EvalAction::NormalBranch(0, smallvec![Value::Felt(value + Felt::from(offset))])
}

fn eval_call_contract(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_storage_read(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system, Value::U32(address_domain), Value::Felt(storage_key)]: [Value; 4] = args.try_into().unwrap() else {
        panic!()
    };
    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let result = syscall_handler.storage_read(address_domain, storage_key, &mut gas);

    match result {
        Ok(value) => {
            EvalAction::NormalBranch(0, smallvec![Value::U128(gas), system, Value::Felt(value)])
        }
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: error_felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_storage_write(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system, Value::U32(address_domain), Value::Felt(storage_key), Value::Felt(value)]: [Value; 5] = args.try_into().unwrap() else {
        panic!()
    };
    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let result = syscall_handler.storage_write(address_domain, storage_key, value, &mut gas);

    match result {
        Ok(_) => EvalAction::NormalBranch(0, smallvec![Value::U128(gas), system]),
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: error_felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_emit_event(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system, Value::Array { ty: _, data: keys }, Value::Array { ty: _, data }]: [Value; 4] = args.try_into().unwrap() else {
        panic!()
    };

    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let keys = keys
        .into_iter()
        .map(|x| match x {
            Value::Felt(x) => x,
            _ => unreachable!(),
        })
        .collect();
    let data = data
        .into_iter()
        .map(|x| match x {
            Value::Felt(x) => x,
            _ => unreachable!(),
        })
        .collect();

    let result = syscall_handler.emit_event(keys, data, &mut gas);

    match result {
        Ok(_) => EvalAction::NormalBranch(0, smallvec![Value::U128(gas), system]),
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: error_felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_get_block_hash(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system, Value::U64(block_number)]: [Value; 3] =
        args.try_into().unwrap()
    else {
        panic!()
    };
    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let result = syscall_handler.get_block_hash(block_number, &mut gas);

    match result {
        Ok(res) => {
            EvalAction::NormalBranch(0, smallvec![Value::U128(gas), system, Value::Felt(res)])
        }
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: error_felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_get_execution_info(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };
    // get felt type from the error branch array
    let felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let result = syscall_handler.get_execution_info(&mut gas);

    match result {
        Ok(res) => EvalAction::NormalBranch(
            0,
            smallvec![Value::U128(gas), system, res.into_value(felt_ty)],
        ),
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_get_execution_info_v2(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let [Value::U128(mut gas), system]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };
    // get felt type from the error branch array
    let felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[2].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    let result = syscall_handler.get_execution_info_v2(&mut gas);

    let out_ty = registry
        .get_type(&info.branch_signatures()[0].vars[2].ty)
        .unwrap();

    match result {
        Ok(res) => EvalAction::NormalBranch(
            0,
            smallvec![Value::U128(gas), system, res.into_value(felt_ty)],
        ),
        Err(e) => EvalAction::NormalBranch(
            1,
            smallvec![
                Value::U128(gas),
                system,
                Value::Array {
                    ty: felt_ty,
                    data: e.into_iter().map(Value::Felt).collect::<Vec<_>>(),
                }
            ],
        ),
    }
}

fn eval_deploy(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_keccak(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_library_call(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_replace_class(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_send_message_to_l1(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}
