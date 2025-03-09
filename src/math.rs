/// Archimedes' constant (π)
pub(crate) const PI: f64 = {
    #[cfg(not(feature = "no-std"))]
    {
        std::f64::consts::PI
    }

    #[cfg(feature = "no-std")]
    #[allow(clippy::approx_constant)]
    {
        3.141_592_653_589_793
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
        if x.is_nan() {
            f64::NAN
        } else if x.is_sign_positive() {
            1.
        } else {
            -1.
        }
    }
}

macro_rules! use_std_or_libm {
    ( $( $func: ident $doc: expr ),+ ) => {
        $(
                #[doc = $doc]
                pub(crate) fn $func(x: f64) -> f64 {
                    #[cfg(not(feature = "no-std"))]
                    { f64::$func(x) }
                    #[cfg(feature = "no-std")]
                    { libm::$func(x) }
                }
        )+
    };
}

use_std_or_libm!(
    cos "Computes the cosine of a number (in radians).",
    sin "Computes the sine of a number (in radians).",
    acos "
        Computes the arccosine of a number. Return value is in radians in
        the range [0, pi] or NaN if the number is outside the range
        [-1, 1].
    ",
    asin "
        Computes the arcsine of a number. Return value is in radians in
        the range [-pi/2, pi/2] or NaN if the number is outside the range
        [-1, 1].
    ",
    sqrt "
        Returns the square root of a number. Returns NaN if `self` is a
        negative number other than `-0.0`.
    "
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[allow(clippy::approx_constant)]
    #[test]
    fn test_pi() {
        assert_relative_eq!(PI, 3.141_592_653_589_793)
    }

    #[test]
    fn test_abs() {
        assert_relative_eq!(abs(0.), 0.);
        assert_relative_eq!(abs(1.2), 1.2);
        assert_relative_eq!(abs(-1.2), 1.2);
        assert!(abs(f64::NAN).is_nan());
    }

    #[test]
    fn test_signum() {
        assert_relative_eq!(signum(1.2), 1.);
        assert_relative_eq!(signum(-1.2), -1.);
        assert_relative_eq!(signum(0.), 1.);
        assert_relative_eq!(signum(-0.), -1.);
        assert!(signum(f64::NAN).is_nan());
    }

    #[test]
    fn test_cos() {
        assert_relative_eq!(cos(PI / 3.), 0.5);
        assert_relative_eq!(cos(2. * PI / 3.), -0.5);
    }

    #[test]
    fn test_acos() {
        assert_relative_eq!(acos(0.5), PI / 3.);
        assert_relative_eq!(acos(-0.5), 2. * PI / 3.);
        assert!(acos(2.).is_nan());
    }

    #[test]
    fn test_sin() {
        assert_relative_eq!(sin(PI / 3.), sqrt(3.) / 2.);
        assert_relative_eq!(sin(2. * PI / 3.), sqrt(3.) / 2.);
    }

    #[test]
    fn test_asin() {
        assert_relative_eq!(asin(0.5), PI / 6.);
        assert_relative_eq!(asin(-0.5), -PI / 6.);
        assert!(asin(2.).is_nan());
    }

    #[test]
    fn test_sqrt() {
        assert_relative_eq!(sqrt(0.), 0.);
        assert_relative_eq!(sqrt(1.), 1.);
        assert_relative_eq!(sqrt(4.), 2.);
        assert!(sqrt(-1.).is_nan());
    }
}
