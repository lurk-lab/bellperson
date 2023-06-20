use crate::{gadgets::{num::AllocatedNum, circom::CircomTemplate}, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use crate::gadgets::circom::sha256::{constrain_equals, value};

pub struct RotR<F: PrimeField> {
    pub n: usize,
    pub r: usize,
    pub __in: Vec<AllocatedNum<F>>,
    pub out: Vec<AllocatedNum<F>>,
}

impl<F: PrimeField> RotR<F> {
    pub fn new(n: usize, r: usize) -> Self {
        RotR {
            n,
            r,
            __in: vec![AllocatedNum::initialize(); n],
            out: vec![AllocatedNum::initialize(); n],
        }
    }
}

impl<F: PrimeField> CircomTemplate<F> for RotR<F> {
    type Target = ();

    fn generate_output_signal<CS>(&mut self, cs: CS) -> Result<Self::Target, SynthesisError>
    where
        CS: ConstraintSystem<F>,
    {
        self.out = rotr(cs, self.n, self.r, &self.__in)?;
        Ok(())
    }
}

pub fn rotr<F: PrimeField, CS: ConstraintSystem<F>>(
    mut cs: CS,
    n: usize,
    r: usize,
    __in: &[AllocatedNum<F>],
) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
    let mut out = vec![AllocatedNum::initialize(); n];
    let mut i = 0;
    while i < n {
        constrain_equals!(cs, out[i], __in[(i + r) % n], "out[{i}]");
        i += 1;
    }

    assert_eq!(out.len(), n, "expected length {}, got {}", n, out.len());
    Ok(out)
}