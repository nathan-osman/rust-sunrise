/// Archimedes' constant (π)
pub(crate) const PI: f64 = {
    #[cfg(not(feature = "no-std"))]
    {
        std::f64::consts::PI
    }

    #[cfg(feature = "no-std")]
    {
        3.141_592_653_589_793_238_462_643_383_279_502_88
    }
};

/// Given an angle 𝛅 given in degrees, 𝛅 * DEGREE is the same angle in radians
pub(crate) const DEGREE: f64 = PI / 180.;

/// Computes the absolute value of `x`.
pub(crate) fn abs(x: f64) -> f64 {
    #[cfg(not(feature = "no-std"))]
    {
        f64::abs(x)
    }

    #[cfg(feature = "no-std")]
    {
        libm::fabs(x)
    }
}

/// Returns a number that represents the sign of `x`.
///
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - NaN if the number is NaN
pub(crate) fn signum(x: f64) -> f64 {
    #[cfg(not(feature = "no-std"))]
    {
        f64::signum(x)
    }

    #[cfg(feature = "no-std")]
    {
        if x.is_sign_positive() {
            1.
        } else if x.is_sign_negative() {
            -1.
        } else {
            f64::NAN
        }
    }
}

macro_rules! use_std_or_libm {
    ( $( $func: ident ),+ ) => {
        $(
            pub(crate) fn $func(x: f64) -> f64 {
                #[cfg(not(feature = "no-std"))]
                { f64::$func(x) }
                #[cfg(feature = "no-std")]
                { libm::$func(x) }
            }
        )+
    };
}

use_std_or_libm!(cos, acos, sin, asin, sqrt);
