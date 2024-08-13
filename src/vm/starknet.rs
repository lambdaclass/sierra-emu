use super::EvalAction;
use crate::{starknet::StarknetSyscallHandler, Value};
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureOnlyConcreteLibfunc,
        starknet::StarkNetConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};

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
        StarkNetConcreteLibfunc::ClassHashConst(_) => todo!(),
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
        StarkNetConcreteLibfunc::StorageBaseAddressConst(_) => todo!(),
        StarkNetConcreteLibfunc::StorageBaseAddressFromFelt252(_) => todo!(),
        StarkNetConcreteLibfunc::StorageAddressFromBase(_) => todo!(),
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
    todo!()
}

fn eval_storage_write(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
    syscall_handler: &mut impl StarknetSyscallHandler,
) -> EvalAction {
    todo!()
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
