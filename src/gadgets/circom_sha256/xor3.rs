use std::vec;

use crate::{gadgets::num::AllocatedNum, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use crate::gadgets::circom_sha256::value;

pub struct XOr3<F: PrimeField> {
    pub n: usize,
    pub a: Vec<AllocatedNum<F>>,
    pub b: Vec<AllocatedNum<F>>,
    pub c: Vec<AllocatedNum<F>>,
    pub out: Vec<AllocatedNum<F>>,
}

impl<F: PrimeField> XOr3<F> {
    pub fn new(n: usize) -> Self {
        XOr3 {
            n,
            a: vec![AllocatedNum::initialize(); n],
            b: vec![AllocatedNum::initialize(); n],
            c: vec![AllocatedNum::initialize(); n],
            out: vec![AllocatedNum::initialize(); n],
        }
    }

    pub fn force<CS: ConstraintSystem<F>>(&mut self, cs: CS) -> Result<(), SynthesisError> {
        self.out = xor3(cs, self.n, &self.a, &self.b, &self.c)?;
        Ok(())
    }
}

/// We translate the circom array type `a[n]` to `Vec<AllocatedNum<F>>` for now
pub fn xor3<F: PrimeField, CS: ConstraintSystem<F>>(
    mut cs: CS,
    n: usize,
    a: &[AllocatedNum<F>],
    b: &[AllocatedNum<F>],
    c: &[AllocatedNum<F>],
) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
    let mut mid = vec![AllocatedNum::initialize(); n];
    let mut out = vec![AllocatedNum::initialize(); n];
    let mut k = 0;
    while k < n {
        mid[k].assign(cs.namespace(|| format!("mid[{k}]")), || {
            let tmp = value!(b[k]) * value!(c[k]);
            Ok(tmp)
        })?;

        cs.enforce(
            || format!("mid[{k}] constraint"),
            |lc| lc + a[k].get_variable(),
            |lc| lc + b[k].get_variable(),
            |lc| lc + mid[k].get_variable(),
        );

        out[k].assign(cs.namespace(|| format!("out[{k}]")), || {
            let tmp = value!(a[k])
                * (F::ONE - F::from(2) * value!(b[k]) - F::from(2) * value!(c[k])
                    + F::from(4) * value!(mid[k]))
                + value!(b[k])
                + value!(c[k])
                - F::from(2) * value!(mid[k]);
            Ok(tmp)
        })?;

        cs.enforce(
            || format!("out[{k}] constraint"),
            |lc| lc + a[k].get_variable(),
            |lc| {
                lc + CS::one()
                    + (-F::from(2), b[k].get_variable())
                    + (-F::from(2), c[k].get_variable())
                    + (F::from(4), mid[k].get_variable())
            },
            |lc| {
                lc + out[k].get_variable()
                    + (-F::from(1), b[k].get_variable())
                    + (-F::from(1), c[k].get_variable())
                    + (F::from(2), mid[k].get_variable())
            },
        );

        k += 1;
    }

    assert_eq!(out.len(), n, "expected length {}, got {}", n, out.len());
    Ok(out)
}
