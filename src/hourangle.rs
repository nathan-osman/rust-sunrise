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

use std::f64::consts::PI;

use crate::event::SolarEvent;
use crate::DEGREE;

/// Calculates the second of the two angles required to locate a point on the
/// celestial sphere in the equatorial coordinate system.
pub fn hour_angle(latitude_deg: f64, declination: f64, event: SolarEvent) -> f64 {
    let latitude = latitude_deg * DEGREE;
    let denominator = f64::cos(latitude) * f64::cos(declination);
    let numerator = f64::cos(PI / 2. + event.angle()) - f64::sin(latitude) * f64::sin(declination);

    let sign = {
        if event.is_morning() {
            -1.
        } else {
            1.
        }
    };

    sign * f64::acos(numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_oposites() {
        assert_relative_eq!(
            hour_angle(32., -22., SolarEvent::Sunrise),
            -hour_angle(32., -22., SolarEvent::Sunset),
        )
    }

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(
            hour_angle(0., -22.97753 * DEGREE, SolarEvent::Sunset),
            90.90516 * DEGREE,
            epsilon = 0.00001
        )
    }
}
