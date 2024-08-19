use core::circuit::{
    RangeCheck96, AddMod, MulMod, u96, CircuitElement, CircuitInput, circuit_add, circuit_sub,
    circuit_mul, circuit_inverse, EvalCircuitTrait, u384, CircuitOutputsTrait, CircuitModulus,
    AddInputResultTrait, CircuitInputs,
};

fn main() {
    let in1 = CircuitElement::<CircuitInput<0>> {};
    let in2 = CircuitElement::<CircuitInput<1>> {};
    let add = circuit_add(in1, in2);

    let modulus = TryInto::<_, CircuitModulus>::try_into([7, 0, 0, 0]).unwrap();
    let outputs = (add,)
        .new_inputs()
        .next([3, 0, 0, 0])
        .next([6, 0, 0, 0])
        .done()
        .eval(modulus)
        .unwrap();
    
    outputs.get_output(in1);
}
