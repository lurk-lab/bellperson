use crate::{gadgets::{num::AllocatedNum, circom::CircomTemplate}, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use crate::gadgets::circom::sha256::{constrain_equals, value};

pub struct Shr<F: PrimeField> {
    pub n: usize,
    pub r: usize,
    pub __in: Vec<AllocatedNum<F>>,
    pub out: Vec<AllocatedNum<F>>,
}

impl<F: PrimeField> Shr<F> {
    pub fn new(n: usize, r: usize) -> Self {
        Shr {
            n,
            r,
            __in: vec![AllocatedNum::initialize(); n],
            out: vec![AllocatedNum::initialize(); n],
        }
    }
}

impl<F: PrimeField> CircomTemplate<F> for Shr<F> {
    type Target = ();

    fn generate_output_signal<CS>(&mut self, cs: CS) -> Result<Self::Target, SynthesisError>
    where
        CS: ConstraintSystem<F>,
    {
        self.out = shr(cs, self.n, self.r, &self.__in)?;
        Ok(())
    }
}

pub fn shr<F: PrimeField, CS: ConstraintSystem<F>>(
    mut cs: CS,
    n: usize,
    r: usize,
    __in: &[AllocatedNum<F>],
) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
    let mut out = vec![AllocatedNum::initialize(); n];
    let mut i = 0;
    while i < n {
        if i + r >= n {
            out[i].assign(cs.namespace(|| format!("out {i}")), || {
                let tmp = F::ZERO;
                Ok(tmp)
            })?;

            cs.enforce(
                || format!("out constraints {i}"),
                |lc| lc, // how to enfore zero??
                |lc| lc + CS::one(),
                |lc| lc + out[i].get_variable(),
            );
        } else {
            constrain_equals!(cs, out[i], __in[i + r], "out[{i}]]");
        }

        i += 1;
    }

    assert_eq!(out.len(), n, "expected length {}, got {}", n, out.len());
    Ok(out)
}
