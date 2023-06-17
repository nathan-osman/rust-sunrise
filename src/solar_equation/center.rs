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

use std::f64;

use crate::DEGREE;

/// Calculates the angular difference between the position of the earth in its
/// elliptical orbit and the position it would occupy in a circular orbit for
/// the given mean anomaly.
pub(crate) fn equation_of_center(solar_anomaly: f64) -> f64 {
    let anomaly_sin = f64::sin(solar_anomaly);
    let anomaly_2_sin = f64::sin(2. * solar_anomaly);
    let anomaly_3_sin = f64::sin(3. * solar_anomaly);
    (1.9148 * anomaly_sin + 0.02 * anomaly_2_sin + 0.0003 * anomaly_3_sin) * DEGREE
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(
            equation_of_center(358.30683 * DEGREE),
            -0.05778 * DEGREE,
            epsilon = 0.00001
        )
    }
}
