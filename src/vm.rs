use crate::Value;
use cairo_lang_sierra::{
    edit_state,
    extensions::core::{CoreConcreteLibfunc, CoreLibfunc, CoreType},
    ids::{ConcreteLibfuncId, FunctionId, VarId},
    program::{GenStatement, Invocation, Program, StatementIdx},
    program_registry::ProgramRegistry,
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use smallvec::SmallVec;
use std::{cell::Cell, sync::Arc};
use tracing::debug;

mod ap_tracking;
mod array;
mod bounded_int;
mod r#box;
mod branch_align;
mod cast;
mod r#const;
mod drop;
mod dup;
mod r#enum;
mod felt252;
mod felt252_dict;
mod felt252_dict_entry;
mod function_call;
mod gas;
mod jump;
mod mem;
mod snapshot_take;
mod starknet;
mod r#struct;
mod uint32;
mod uint8;

pub struct VirtualMachine {
    program: Arc<Program>,
    registry: ProgramRegistry<CoreType, CoreLibfunc>,

    frames: Vec<SierraFrame>,
}

impl VirtualMachine {
    pub fn new(program: Arc<Program>) -> Self {
        let registry = ProgramRegistry::new(&program).unwrap();
        Self {
            program,
            registry,

            frames: Vec::new(),
        }
    }

    pub fn registry(&self) -> &ProgramRegistry<CoreType, CoreLibfunc> {
        &self.registry
    }

    /// Effectively a function call (for entry points).
    pub fn push_frame<I>(&mut self, function_id: FunctionId, args: I)
    where
        I: IntoIterator<Item = Value>,
        I::IntoIter: ExactSizeIterator,
    {
        let function = self.registry.get_function(&function_id).unwrap();

        let args = args.into_iter();
        assert_eq!(args.len(), function.params.len());
        self.frames.push(SierraFrame {
            _function_id: function_id,
            state: Cell::new(
                function
                    .params
                    .iter()
                    .zip(args)
                    .map(|(param, value)| {
                        assert!(value.is(&self.registry, &param.ty));
                        (param.id.clone(), value)
                    })
                    .collect(),
            ),
            pc: function.entry_point,
        })
    }

    /// Run a single statement and return the state before its execution.
    pub fn step(&mut self) -> Option<(StatementIdx, OrderedHashMap<VarId, Value>)> {
        let frame = self.frames.last_mut()?;

        let pc_snapshot = frame.pc;
        let state_snapshot = frame.state.get_mut().clone();

        debug!(
            "Evaluating statement {} ({})",
            frame.pc.0, &self.program.statements[frame.pc.0]
        );
        match &self.program.statements[frame.pc.0] {
            GenStatement::Invocation(invocation) => {
                let (state, values) =
                    edit_state::take_args(frame.state.take(), invocation.args.iter()).unwrap();

                match eval(&self.registry, &invocation.libfunc_id, values) {
                    EvalAction::NormalBranch(branch_idx, results) => {
                        assert_eq!(
                            results.len(),
                            invocation.branches[branch_idx].results.len(),
                            "invocation of {invocation} returned the wrong number of values"
                        );

                        frame.pc = frame.pc.next(&invocation.branches[branch_idx].target);
                        frame.state.set(
                            edit_state::put_results(
                                state,
                                invocation.branches[branch_idx].results.iter().zip(results),
                            )
                            .unwrap(),
                        );
                    }
                    EvalAction::FunctionCall(function_id, args) => {
                        let function = self.registry.get_function(&function_id).unwrap();
                        frame.state.set(state);
                        self.frames.push(SierraFrame {
                            _function_id: function_id,
                            state: Cell::new(
                                function
                                    .params
                                    .iter()
                                    .map(|param| param.id.clone())
                                    .zip(args.iter().cloned())
                                    .collect(),
                            ),
                            pc: function.entry_point,
                        });
                    }
                }
            }
            GenStatement::Return(ids) => {
                let curr_frame = self.frames.pop().unwrap();
                if let Some(prev_frame) = self.frames.last_mut() {
                    let (state, values) =
                        edit_state::take_args(curr_frame.state.take(), ids.iter()).unwrap();
                    assert!(state.is_empty());

                    let target_branch = match &self.program.statements[prev_frame.pc.0] {
                        GenStatement::Invocation(Invocation { branches, .. }) => {
                            assert_eq!(branches.len(), 1);
                            &branches[0]
                        }
                        _ => unreachable!(),
                    };

                    assert_eq!(target_branch.results.len(), values.len());
                    prev_frame.pc = prev_frame.pc.next(&target_branch.target);
                    prev_frame.state.set(
                        edit_state::put_results(
                            prev_frame.state.take(),
                            target_branch.results.iter().zip(values),
                        )
                        .unwrap(),
                    );
                }
            }
        }

        Some((pc_snapshot, state_snapshot))
    }
}

struct SierraFrame {
    _function_id: FunctionId,

    state: Cell<OrderedHashMap<VarId, Value>>,
    pc: StatementIdx,
}

enum EvalAction {
    NormalBranch(usize, SmallVec<[Value; 2]>),
    FunctionCall(FunctionId, SmallVec<[Value; 2]>),
}

fn eval<'a>(
    registry: &'a ProgramRegistry<CoreType, CoreLibfunc>,
    id: &'a ConcreteLibfuncId,
    args: Vec<Value>,
) -> EvalAction {
    match registry.get_libfunc(id).unwrap() {
        CoreConcreteLibfunc::ApTracking(selector) => {
            self::ap_tracking::eval(registry, selector, args)
        }
        CoreConcreteLibfunc::Array(selector) => self::array::eval(registry, selector, args),
        CoreConcreteLibfunc::Bool(_) => todo!(),
        CoreConcreteLibfunc::BoundedInt(selector) => {
            self::bounded_int::eval(registry, selector, args)
        }
        CoreConcreteLibfunc::Box(selector) => self::r#box::eval(registry, selector, args),
        CoreConcreteLibfunc::BranchAlign(info) => self::branch_align::eval(registry, info, args),
        CoreConcreteLibfunc::Bytes31(_) => todo!(),
        CoreConcreteLibfunc::Cast(selector) => self::cast::eval(registry, selector, args),
        CoreConcreteLibfunc::Circuit(_) => todo!(),
        CoreConcreteLibfunc::Const(selector) => self::r#const::eval(registry, selector, args),
        CoreConcreteLibfunc::Coupon(_) => todo!(),
        CoreConcreteLibfunc::CouponCall(_) => todo!(),
        CoreConcreteLibfunc::Debug(_) => todo!(),
        CoreConcreteLibfunc::Drop(info) => self::drop::eval(registry, info, args),
        CoreConcreteLibfunc::Dup(info) => self::dup::eval(registry, info, args),
        CoreConcreteLibfunc::Ec(_) => todo!(),
        CoreConcreteLibfunc::Enum(selector) => self::r#enum::eval(registry, selector, args),
        CoreConcreteLibfunc::Felt252(selector) => self::felt252::eval(registry, selector, args),
        CoreConcreteLibfunc::Felt252Dict(selector) => {
            self::felt252_dict::eval(registry, selector, args)
        }
        CoreConcreteLibfunc::Felt252DictEntry(selector) => {
            self::felt252_dict_entry::eval(registry, selector, args)
        }
        CoreConcreteLibfunc::FunctionCall(info) => self::function_call::eval(registry, info, args),
        CoreConcreteLibfunc::Gas(selector) => self::gas::eval(registry, selector, args),
        CoreConcreteLibfunc::Mem(selector) => self::mem::eval(registry, selector, args),
        CoreConcreteLibfunc::Nullable(_) => todo!(),
        CoreConcreteLibfunc::Pedersen(_) => todo!(),
        CoreConcreteLibfunc::Poseidon(_) => todo!(),
        CoreConcreteLibfunc::Sint128(_) => todo!(),
        CoreConcreteLibfunc::Sint16(_) => todo!(),
        CoreConcreteLibfunc::Sint32(_) => todo!(),
        CoreConcreteLibfunc::Sint64(_) => todo!(),
        CoreConcreteLibfunc::Sint8(_) => todo!(),
        CoreConcreteLibfunc::SnapshotTake(info) => self::snapshot_take::eval(registry, info, args),
        CoreConcreteLibfunc::StarkNet(selector) => self::starknet::eval(registry, selector, args),
        CoreConcreteLibfunc::Struct(selector) => self::r#struct::eval(registry, selector, args),
        CoreConcreteLibfunc::Uint128(_) => todo!(),
        CoreConcreteLibfunc::Uint16(_) => todo!(),
        CoreConcreteLibfunc::Uint256(_) => todo!(),
        CoreConcreteLibfunc::Uint32(selector) => self::uint32::eval(registry, selector, args),
        CoreConcreteLibfunc::Uint512(_) => todo!(),
        CoreConcreteLibfunc::Uint64(_) => todo!(),
        CoreConcreteLibfunc::Uint8(selector) => self::uint8::eval(registry, selector, args),
        CoreConcreteLibfunc::UnconditionalJump(info) => self::jump::eval(registry, info, args),
        CoreConcreteLibfunc::UnwrapNonZero(_) => todo!(),
    }
}
