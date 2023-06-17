// The MIT License (MIT)
//
// Copyright (c) 2018 Nathan Osman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use crate::math::{asin, sin};

/// Declination calculates one of the two angles required to locate a point on
/// the celestial sphere in the equatorial coordinate system. The ecliptic
/// longitude parameter must be in degrees.
pub(crate) fn declination(ecliptic_longitude: f64) -> f64 {
    asin(sin(ecliptic_longitude) * 0.39779)
}

#[cfg(test)]
mod tests {
    use crate::math::DEGREE;
    use approx::assert_relative_eq;

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(
            super::declination(281.08372 * DEGREE),
            -22.97753 * DEGREE,
            epsilon = 0.00001
        )
    }
}
