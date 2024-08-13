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
        StarkNetConcreteLibfunc::ClassHashTryFromFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::ClassHashToFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::ContractAddressConst(_) => todo!(),
        StarkNetConcreteLibfunc::ContractAddressTryFromFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::ContractAddressToFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::StorageRead(info) => {
            eval_storage_read(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::StorageWrite(info) => {
            eval_storage_write(registry, info, args, syscall_handler)
        }
        StarkNetConcreteLibfunc::StorageBaseAddressConst(info) => {
            eval_storage_base_address_const(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageBaseAddressFromFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::StorageAddressFromBase(info) => {
            eval_storage_address_from_base(registry, info, args)
        }
        StarkNetConcreteLibfunc::StorageAddressFromBaseAndOffset(_) => todo!(),
        StarkNetConcreteLibfunc::StorageAddressToFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::StorageAddressTryFromFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::EmitEvent(info) => eval_emit_event(registry, info, args),
        StarkNetConcreteLibfunc::GetBlockHash(info) => eval_get_block_hash(registry, info, args),
        StarkNetConcreteLibfunc::GetExecutionInfo(info) => {
            eval_get_execution_info(registry, info, args)
        }
        StarkNetConcreteLibfunc::GetExecutionInfoV2(info) => {
            eval_get_execution_info_v2(registry, info, args)
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

fn eval_storage_address_from_base(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();
    EvalAction::NormalBranch(0, smallvec![value])
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
    let args: [Value; 4] = args.try_into().unwrap();
    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[0].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    match args {
        [Value::U128(mut gas), system, Value::U32(address_domain), Value::Felt(storage_key)] => {
            let result = syscall_handler.storage_read(address_domain, storage_key, &mut gas);

            match result {
                Ok(value) => EvalAction::NormalBranch(
                    0,
                    smallvec![Value::U128(gas), system, Value::Felt(value)],
                ),
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
        _ => panic!(),
    }
}

fn eval_storage_write(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    let args: [Value; 5] = args.try_into().unwrap();
    let error_felt_ty = {
        match registry
            .get_type(&info.branch_signatures()[1].vars[0].ty)
            .unwrap()
        {
            CoreTypeConcrete::Array(info) => info.ty.clone(),
            _ => unreachable!(),
        }
    };

    match args {
        [Value::U128(mut gas), system, Value::U32(address_domain), Value::Felt(storage_key), Value::Felt(value)] =>
        {
            let result =
                syscall_handler.storage_write(address_domain, storage_key, value, &mut gas);

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
        _ => panic!(),
    }
}

fn eval_emit_event(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_get_block_hash(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_get_execution_info(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
}

fn eval_get_execution_info_v2(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    todo!()
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
