use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        core::{CoreLibfunc, CoreType},
        ec::EcConcreteLibfunc,
        lib_func::SignatureOnlyConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use num_traits::identities::Zero;
use smallvec::smallvec;
use starknet_crypto::Felt;
use starknet_curve::curve_params::BETA;
use starknet_types_core::curve::{AffinePoint, ProjectivePoint};

// todo: verify these are correct.

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &EcConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        EcConcreteLibfunc::IsZero(info) => eval_is_zero(registry, info, args),
        EcConcreteLibfunc::Neg(_) => todo!(),
        EcConcreteLibfunc::StateAdd(info) => eval_state_add(registry, info, args),
        EcConcreteLibfunc::TryNew(info) => eval_new(registry, info, args),
        EcConcreteLibfunc::StateFinalize(_) => todo!(),
        EcConcreteLibfunc::StateInit(_) => todo!(),
        EcConcreteLibfunc::StateAddMul(_) => todo!(),
        EcConcreteLibfunc::PointFromX(info) => eval_point_from_x(registry, info, args),
        EcConcreteLibfunc::UnwrapPoint(_) => todo!(),
        EcConcreteLibfunc::Zero(_) => todo!(),
    }
}

pub fn eval_is_zero(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value @ Value::EcPoint { x, y }]: [Value; 1] = args.try_into().unwrap() else {
        panic!()
    };

    if x.is_zero() && y.is_zero() {
        EvalAction::NormalBranch(0, smallvec![])
    } else {
        EvalAction::NormalBranch(1, smallvec![value])
    }
}

pub fn eval_new(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::Felt(x), Value::Felt(y)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    match AffinePoint::new(x, y) {
        Ok(point) => EvalAction::NormalBranch(
            0,
            smallvec![Value::EcPoint {
                x: point.x(),
                y: point.y(),
            }],
        ),
        Err(_) => EvalAction::NormalBranch(1, smallvec![]),
    }
}

pub fn eval_state_add(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [Value::EcState { x0, y0, x1, y1 }, Value::EcPoint { x, y }]: [Value; 2] =
        args.try_into().unwrap()
    else {
        panic!()
    };

    let mut state = ProjectivePoint::from_affine(x0, y0).unwrap();
    let point = AffinePoint::new(x, y).unwrap();

    state += &point;

    EvalAction::NormalBranch(
        0,
        smallvec![Value::EcState {
            x0: state.x(),
            y0: state.y(),
            x1,
            y1
        }],
    )
}

pub fn eval_point_from_x(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureOnlyConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [range_check @ Value::Unit, Value::Felt(x)]: [Value; 2] = args.try_into().unwrap() else {
        panic!()
    };

    // https://github.com/starkware-libs/cairo/blob/aaad921bba52e729dc24ece07fab2edf09ccfa15/crates/cairo-lang-sierra-to-casm/src/invocations/ec.rs#L63
    let x2 = x * x;
    let x3 = x2 * x;
    let alpha_x_plus_beta = x + BETA;
    let rhs = x3 + alpha_x_plus_beta;
    let y = rhs.sqrt().unwrap_or_else(|| Felt::from(3) * rhs);

    match AffinePoint::new(x, y) {
        Ok(point) => EvalAction::NormalBranch(
            0,
            smallvec![
                range_check,
                Value::EcPoint {
                    x: point.x(),
                    y: point.y(),
                }
            ],
        ),
        Err(_) => EvalAction::NormalBranch(1, smallvec![range_check]),
    }
}
