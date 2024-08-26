## The plonk algorithm for lambdaworks

```shell
use lambdaworks_plonk::constraint_system::ConstraintSystem;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrField;

fn main() {
    let system = &mut ConstraintSystem::<FrField>::new();
    let x = system.new_public_input();
    let y = system.new_public_input();
    let e = system.new_variable();

    let z = system.mul(&x, &e);
    
    // This constraint system asserts that x * e == y
    system.assert_eq(&y, &z);
}
```
