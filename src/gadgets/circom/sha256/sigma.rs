use crate::{
    gadgets::{circom::CircomTemplate, num::AllocatedNum},
    ConstraintSystem, SynthesisError,
};
use ff::PrimeField;

use crate::gadgets::circom::sha256::{
    constrain_equals, rotate::RotR, shift::Shr, value, xor3::XOr3,
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
}

impl<F: PrimeField> CircomTemplate<F> for SmallSigma<F> {
    type Target = ();

    fn generate_output_signal<CS>(&mut self, cs: CS) -> Result<Self::Target, SynthesisError>
    where
        CS: ConstraintSystem<F>,
    {
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

    rota.generate_output_signal(cs.namespace(|| "rota"))?;
    rotb.generate_output_signal(cs.namespace(|| "rotb"))?;
    shrc.generate_output_signal(cs.namespace(|| "shrc"))?;

    let mut xor3: XOr3<F> = XOr3::new(32);

    k = 0;
    while k < 32 {
        constrain_equals!(cs, xor3.a[k], rota.out[k], "xor3.a[{k}]");
        constrain_equals!(cs, xor3.b[k], rotb.out[k], "xor3.b[{k}]");
        constrain_equals!(cs, xor3.c[k], shrc.out[k], "xor3.c[{k}]");
        k += 1;
    }

    xor3.generate_output_signal(cs.namespace(|| "xor3"))?;

    k = 0;
    while k < 32 {
        constrain_equals!(cs, out[k], xor3.out[k], "out[{k}]");
        k += 1;
    }

    assert_eq!(out.len(), 32, "expected length 32, got {}", out.len());
    Ok(out)
}
