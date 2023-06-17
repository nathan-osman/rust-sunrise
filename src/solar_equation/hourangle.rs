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

use crate::event::SolarEvent;
use crate::math::{abs, acos, cos, signum, sin, sqrt, DEGREE};

/// Calculates the second of the two angles required to locate a point on the
/// celestial sphere in the equatorial coordinate system.
pub(crate) fn hour_angle(
    latitude_deg: f64,
    declination: f64,
    altitude: f64,
    event: SolarEvent,
) -> f64 {
    let latitude = latitude_deg * DEGREE;
    let denominator = cos(latitude) * cos(declination);

    let numerator =
        -sin(event.angle() + (2.076 * DEGREE * signum(altitude) * sqrt(abs(altitude)) / 60.))
            - sin(latitude) * sin(declination);

    let sign = {
        if event.is_morning() {
            -1.
        } else {
            1.
        }
    };

    sign * acos(numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_oposites() {
        assert_relative_eq!(
            hour_angle(32., -22., 0., SolarEvent::Sunrise),
            -hour_angle(32., -22., 0., SolarEvent::Sunset),
        );
    }

    #[test]
    fn test_prime_meridian() {
        assert_relative_eq!(
            hour_angle(0., -22.97753 * DEGREE, 0., SolarEvent::Sunset),
            90.90516 * DEGREE,
            epsilon = 0.00001
        );
    }

    #[test]
    fn test_altitude() {
        assert_relative_eq!(
            hour_angle(0., -22.97753 * DEGREE, 100., SolarEvent::Sunset),
            91.28098 * DEGREE,
            epsilon = 0.00001
        );

        assert_relative_eq!(
            hour_angle(0., -22.97753 * DEGREE, -100., SolarEvent::Sunset),
            90.52933 * DEGREE,
            epsilon = 0.00001
        );
    }
}
