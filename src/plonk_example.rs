use lambdaworks_plonk::{
    constraint_system::{ConstraintSystem,Variable},
    prover::Prover,
    setup::{CommonPreprocessedInput,setup,Witness},
    test_utils::utils::{test_srs, TestRandomFieldGenerator, KZG, ORDER_R_MINUS_1_ROOT_UNITY},
    verifier::Verifier,
};

use lambdaworks_math::{
    field::element::FieldElement,
    elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField
};

use std::collections::HashMap;
#[allow(unused)]
fn plonk_example() {
    let system = &mut ConstraintSystem::<FrField>::new();
    let x = system.new_public_input();
    let y = system.new_public_input();
    let e = system.new_variable();

    let z = system.mul(&x, &e);

    system.assert_eq(&y, &z);
}

pub fn plonk_proof_verify() {

    // setup and obtain vk
    let system = &mut ConstraintSystem::<FrField>::new();
    let x = system.new_public_input();
    let e = system.new_variable();

    let common =
        CommonPreprocessedInput::from_constraint_system(system, &ORDER_R_MINUS_1_ROOT_UNITY);
    let srs = test_srs(common.n);
    let kzg = KZG::new(srs); // The commitment scheme for plonk.
    let vk = setup(&common, &kzg);

    // generate proofs
    let inputs = HashMap::from([(x, FieldElement::from(4)), (e, FieldElement::from(3))]);
    let assignments = system.solve(inputs).unwrap();
    let witness = Witness::new(assignments.clone(), &system);

    let public_inputs = system.public_input_values(&assignments);
    let prover = Prover::new(kzg.clone(), TestRandomFieldGenerator {});
    let proof = prover.prove(&witness, &public_inputs, &common, &vk);

    let verifier = Verifier::new(kzg);
    assert!(verifier.verify(&proof, &public_inputs, &common, &vk));
}

#[allow(unused)]
// complex system
fn pow(
    system: &mut ConstraintSystem<FrField>,
    base: Variable,
    exponent: Variable,
)-> Variable{
    let exponent_bits = system.new_u32(&exponent);
    let mut result = system.new_constant(FieldElement::one());

    for i in 0..32 {
        if i != 0 {
            result = system.mul(&result, &result);
        }
        let result_times_base = system.mul(&result, &base);
        result = system.if_else(&exponent_bits[i], &result_times_base, &result);
    }
    result
}

mod test {

    #[allow(unused_imports)]
    use super::{pow, plonk_example, plonk_proof_verify};

    #[allow(unused_imports)]
    use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField;
    #[allow(unused_imports)]
    use lambdaworks_plonk::constraint_system::ConstraintSystem;

    #[test]
    fn test_plonk() {
        plonk_example();
    }

    #[test]
    fn test_plonk_proof() {
        plonk_proof_verify();
    }

    #[test]
    fn test_complex_system(){
        let system = &mut ConstraintSystem::<FrField>::new();
        let x = system.new_public_input();
        let y = system.new_public_input();
        let e = system.new_variable();

        let z = pow(system, x, e);
        system.assert_eq(&y, &z);
    }
}
