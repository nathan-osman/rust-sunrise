// Calculate the non-negative remainder of `lhs mod rhs`.
pub(crate) fn rem_euclid(lhs: f64, rhs: f64) -> f64 {
    #[cfg(not(feature = "no-std"))]
    {
        lhs.rem_euclid(rhs)
    }

    #[cfg(feature = "no-std")]
    {
        let res = lhs % rhs;
        if res < 0. { res + rhs.abs() } else { res }
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
    use core::f64::consts::PI;

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_rem_euclid() {
        assert_relative_eq!(rem_euclid(10.0, 3.0), 1.0);
        assert_relative_eq!(rem_euclid(-8.0, 5.0), 2.0);
        assert_relative_eq!(rem_euclid(-12.0, 5.0), 3.0);
        assert_relative_eq!(rem_euclid(4.0, 4.0), 0.0);
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
