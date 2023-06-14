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

use crate::DEGREE;

/// Calculates the second of the two angles required to locate a point on the
/// celestial sphere in the equatorial coordinate system.
pub fn hour_angle(latitude: f64, declination: f64) -> f64 {
    let latitude_rad = latitude * DEGREE;
    let declination_rad = declination * DEGREE;
    let numerator = -0.01449 - f64::sin(latitude_rad) * f64::sin(declination_rad);
    let denominator = f64::cos(latitude_rad) * f64::cos(declination_rad);
    f64::acos(numerator / denominator) / DEGREE
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(super::hour_angle(0., -22.97753), 90.9018, epsilon = 0.00001)
    }
}
