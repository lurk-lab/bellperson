pub mod sha256;

use ff::PrimeField;

use crate::{ConstraintSystem, SynthesisError};

/// A `CircomTemplate` represents a template instance from circom.
pub trait CircomTemplate<F: PrimeField> {
    type Target;

    /// If all of the inputs signals of the template instance is satisfied,
    /// run the template's logic and set the output signals.
    /// This also synthesizes constraints at the same time.
    /// 
    /// # Panics
    /// 
    /// Panics if the input signals are not all assigned.
    fn generate_output_signal<CS>(&mut self, cs: CS) -> Result<Self::Target, SynthesisError>
    where
        CS: ConstraintSystem<F>;
}