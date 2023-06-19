pub mod xor3;
pub mod rotate;
pub mod shift;
pub mod sigma;

macro_rules! value {
    ($e:expr) => {
        $e.get_value().ok_or(SynthesisError::AssignmentMissing)?
    };
}

macro_rules! constrain_equals {
    ($cs:ident, $e1:expr, $e2:expr, $str:expr) => {
        $e1.assign($cs.namespace(|| format!($str)), || {
            let tmp = value!($e2);
            Ok(tmp)
        })?;

        $cs.enforce(
            || format!("{} constraint", $str),
            |lc| lc + $e2.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + $e1.get_variable(),
        );
    }
}

pub(crate) use value;
pub(crate) use constrain_equals;