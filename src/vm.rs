use crate::value::Value;
use cairo_lang_sierra::{
    edit_state,
    extensions::core::{CoreConcreteLibfunc, CoreLibfunc, CoreType},
    ids::{ConcreteLibfuncId, FunctionId, VarId},
    program::{GenStatement, Invocation, Program, StatementIdx},
    program_registry::ProgramRegistry,
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use std::cell::Cell;

mod r#const;
mod mem;

pub struct VirtualMachine<'a> {
    program: &'a Program,
    registry: ProgramRegistry<CoreType, CoreLibfunc>,

    frames: Vec<SierraFrame<'a>>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(program: &'a Program) -> Self {
        let registry = ProgramRegistry::new(program).unwrap();

        Self {
            program,
            registry,

            frames: Vec::new(),
        }
    }

    /// Effectively a function call (for entry points).
    pub fn push_frame<I>(&mut self, function_id: &'a FunctionId, args: I)
    where
        I: IntoIterator<Item = Value>,
        I::IntoIter: ExactSizeIterator,
    {
        let function = self.registry.get_function(function_id).unwrap();

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
                        assert!(value.is(self.registry.get_type(&param.ty).unwrap()));
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

        match &self.program.statements[frame.pc.0] {
            GenStatement::Invocation(invocation) => {
                let (state, values) =
                    edit_state::take_args(frame.state.take(), invocation.args.iter()).unwrap();

                let (branch_idx, results) = eval(&self.registry, &invocation.libfunc_id, &values);
                if let Some(branch_idx) = branch_idx {
                    frame.pc = frame.pc.next(&invocation.branches[branch_idx].target);
                    frame.state.set(
                        edit_state::put_results(
                            state,
                            invocation.branches[branch_idx].results.iter().zip(results),
                        )
                        .unwrap(),
                    );
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

struct SierraFrame<'a> {
    _function_id: &'a FunctionId,

    state: Cell<OrderedHashMap<VarId, Value>>,
    pc: StatementIdx,
}

fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    id: &ConcreteLibfuncId,
    args: &[Value],
) -> (Option<usize>, Vec<Value>) {
    match registry.get_libfunc(id).unwrap() {
        CoreConcreteLibfunc::ApTracking(_) => todo!(),
        CoreConcreteLibfunc::Array(_) => todo!(),
        CoreConcreteLibfunc::Bool(_) => todo!(),
        CoreConcreteLibfunc::BoundedInt(_) => todo!(),
        CoreConcreteLibfunc::Box(_) => todo!(),
        CoreConcreteLibfunc::BranchAlign(_) => todo!(),
        CoreConcreteLibfunc::Bytes31(_) => todo!(),
        CoreConcreteLibfunc::Cast(_) => todo!(),
        CoreConcreteLibfunc::Circuit(_) => todo!(),
        CoreConcreteLibfunc::Const(selector) => self::r#const::eval(registry, selector, args),
        CoreConcreteLibfunc::Coupon(_) => todo!(),
        CoreConcreteLibfunc::CouponCall(_) => todo!(),
        CoreConcreteLibfunc::Debug(_) => todo!(),
        CoreConcreteLibfunc::Drop(_) => todo!(),
        CoreConcreteLibfunc::Dup(_) => todo!(),
        CoreConcreteLibfunc::Ec(_) => todo!(),
        CoreConcreteLibfunc::Enum(_) => todo!(),
        CoreConcreteLibfunc::Felt252(_) => todo!(),
        CoreConcreteLibfunc::Felt252Dict(_) => todo!(),
        CoreConcreteLibfunc::Felt252DictEntry(_) => todo!(),
        CoreConcreteLibfunc::FunctionCall(_) => todo!(),
        CoreConcreteLibfunc::Gas(_) => todo!(),
        CoreConcreteLibfunc::Mem(selector) => self::mem::eval(registry, selector, args),
        CoreConcreteLibfunc::Nullable(_) => todo!(),
        CoreConcreteLibfunc::Pedersen(_) => todo!(),
        CoreConcreteLibfunc::Poseidon(_) => todo!(),
        CoreConcreteLibfunc::Sint128(_) => todo!(),
        CoreConcreteLibfunc::Sint16(_) => todo!(),
        CoreConcreteLibfunc::Sint32(_) => todo!(),
        CoreConcreteLibfunc::Sint64(_) => todo!(),
        CoreConcreteLibfunc::Sint8(_) => todo!(),
        CoreConcreteLibfunc::SnapshotTake(_) => todo!(),
        CoreConcreteLibfunc::StarkNet(_) => todo!(),
        CoreConcreteLibfunc::Struct(_) => todo!(),
        CoreConcreteLibfunc::Uint128(_) => todo!(),
        CoreConcreteLibfunc::Uint16(_) => todo!(),
        CoreConcreteLibfunc::Uint256(_) => todo!(),
        CoreConcreteLibfunc::Uint32(_) => todo!(),
        CoreConcreteLibfunc::Uint512(_) => todo!(),
        CoreConcreteLibfunc::Uint64(_) => todo!(),
        CoreConcreteLibfunc::Uint8(_) => todo!(),
        CoreConcreteLibfunc::UnconditionalJump(_) => todo!(),
        CoreConcreteLibfunc::UnwrapNonZero(_) => todo!(),
    }
}
