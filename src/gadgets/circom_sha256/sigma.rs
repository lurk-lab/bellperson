use crate::{gadgets::num::AllocatedNum, ConstraintSystem, SynthesisError};
use ff::PrimeField;

use crate::gadgets::circom_sha256::{
    value, shift::Shr, rotate::RotR, constrain_equals, xor3::XOr3,
};

struct SmallSigma<F: PrimeField> {
    pub ra: usize,
    pub rb: usize,
    pub rc: usize,
    pub __in: Vec<AllocatedNum<F>>,
    pub out: Vec<AllocatedNum<F>>,
}

impl<F: PrimeField> SmallSigma<F> {
    pub fn new(ra: usize, rb: usize, rc: usize) -> Self {
        SmallSigma {
            ra,
            rb,
            rc,
            __in: vec![AllocatedNum::initialize(); 32],
            out: vec![AllocatedNum::initialize(); 32],
        }
    }

    pub fn force<CS: ConstraintSystem<F>>(mut self, cs: CS) -> Result<(), SynthesisError> {
        self.out = small_sigma(cs, self.ra, self.rb, self.rc, &self.__in)?;
        Ok(())
    }
}
pub fn small_sigma<F: PrimeField, CS: ConstraintSystem<F>>(
    mut cs: CS,
    ra: usize,
    rb: usize,
    rc: usize,
    __in: &[AllocatedNum<F>],
) -> Result<Vec<AllocatedNum<F>>, SynthesisError> {
    let mut out = vec![AllocatedNum::initialize(); 32];

    let mut k;

    let mut rota: RotR<F> = RotR::new(32, ra);
    let mut rotb: RotR<F> = RotR::new(32, rb);
    let mut shrc: Shr<F> = Shr::new(32, rc);

    k = 0;
    while k < 32 {
        constrain_equals!(cs, rota.__in[k], __in[k], "rota.in[{k}]");
        constrain_equals!(cs, rotb.__in[k], __in[k], "rotb.in[{k}]");
        constrain_equals!(cs, shrc.__in[k], __in[k], "shrc.in[{k}]");
        k += 1;
    }

    rota.force(cs.namespace(|| "rota"))?;
    rotb.force(cs.namespace(|| "rotb"))?;
    shrc.force(cs.namespace(|| "shrc"))?;

    let mut xor3: XOr3<F> = XOr3::new(32);

    k = 0;
    while k < 32 {
        constrain_equals!(cs, xor3.a[k], rota.out[k], "xor3.a[{k}]");
        constrain_equals!(cs, xor3.b[k], rotb.out[k], "xor3.b[{k}]");
        constrain_equals!(cs, xor3.c[k], shrc.out[k], "xor3.c[{k}]");
        k += 1;
    }

    xor3.force(cs.namespace(|| "xor3"))?;

    k = 0;
    while k < 32 {
        constrain_equals!(cs, out[k], xor3.out[k], "out[{k}]");
        k += 1;
    }

    assert_eq!(out.len(), 32, "expected length 32, got {}", out.len());
    Ok(out)
}
