pub mod rotate;
pub mod shift;
pub mod sigma;
pub mod xor3;

/// Quick macro to avoid all the clutter of `AllocatedNum<F> -> F`.
macro_rules! value {
    ($e:expr) => {
        $e.get_value().ok_or(SynthesisError::AssignmentMissing)?
    };
}

/// Quick macro to represent `e1 <== e2;` from circom.
macro_rules! constrain_equals {
    ($cs:ident, $e1:expr, $e2:expr, $str:expr) => {
        $e1.assign($cs.namespace(|| format!($str)), || {
            let tmp = value!($e2);
            Ok(tmp)
        })?;

        $cs.enforce(
            || format!("{} constraint", format!($str)),
            |lc| lc + $e2.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + $e1.get_variable(),
        );
    };
}

pub(crate) use constrain_equals;
pub(crate) use value;
